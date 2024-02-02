use crate::package::{DependencyType, Package};
use crate::package_graph_error::PackageGraphError;
use clean_path::Clean;
use node_package_json::{
    DependenciesMap, PackageJson, Version, VersionProtocol, WorkspaceProtocol, WorkspacesField,
};
use node_package_managers::{pnpm::PnpmWorkspaceYaml, PackageManager};
use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use starbase_utils::{glob, json, yaml};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub type PackageGraphType = DiGraph<String, DependencyType>;

pub struct PackageGraph {
    pub cwd: PathBuf,
    pub manager: PackageManager,
    pub packages: BTreeMap<String, Package>,
    pub root: PathBuf,
    pub root_package: Package,

    graph: PackageGraphType,
    package_globs: Vec<String>,
}

impl PackageGraph {
    pub fn generate<T: AsRef<Path>>(working_dir: T) -> Result<PackageGraph, PackageGraphError> {
        let mut graph = Self::load_from(working_dir)?;
        graph.load_workspace_packages()?;
        graph.generate_graph()?;

        Ok(graph)
    }

    pub fn load_from<T: AsRef<Path>>(working_dir: T) -> Result<PackageGraph, PackageGraphError> {
        let working_dir = working_dir.as_ref();

        // Find the root package.json
        let (root, package_manager) = Self::find_package_root(working_dir)
            .unwrap_or_else(|| (working_dir.to_owned(), PackageManager::Npm));

        let root_manifest: PackageJson = json::read_file(root.join("package.json"))?;

        // Extract workspaces globs
        let mut package_globs = vec![];

        if package_manager == PackageManager::Pnpm {
            let ws_file = root.join("pnpm-workspace.yaml");

            if ws_file.exists() {
                let ws: PnpmWorkspaceYaml = yaml::read_file(ws_file)?;

                package_globs = ws.packages;
            }
        } else if let Some(workspaces) = &root_manifest.workspaces {
            package_globs = match workspaces {
                WorkspacesField::Globs(globs) => globs.to_owned(),
                WorkspacesField::Config { packages, .. } => packages.to_owned(),
            };
        }

        Ok(PackageGraph {
            cwd: working_dir.to_owned(),
            graph: DiGraph::new(),
            manager: package_manager,
            package_globs,
            packages: BTreeMap::new(),
            root_package: Package::new(root.clone(), root_manifest),
            root,
        })
    }

    pub fn find_package_root<T: AsRef<Path>>(starting_dir: T) -> Option<(PathBuf, PackageManager)> {
        let starting_dir = starting_dir.as_ref();
        let mut current_dir = Some(starting_dir);

        while let Some(dir) = current_dir {
            // bun
            if dir.join("bun.lockb").exists() {
                return Some((dir.to_owned(), PackageManager::Bun));
            }
            // pnpm
            else if dir.join("pnpm-lock.yaml").exists()
                || dir.join("pnpm-workspace.yaml").exists()
            {
                return Some((dir.to_owned(), PackageManager::Pnpm));
            }
            // yarn
            else if dir.join("yarn.lock").exists() {
                return Some((
                    dir.to_owned(),
                    if dir.join(".yarn").exists() || dir.join(".yarnrc.yml").exists() {
                        PackageManager::Yarn
                    } else {
                        PackageManager::YarnLegacy
                    },
                ));
            }
            // npm
            else if dir.join("package-lock.json").exists()
                || dir.join("npm-shrinkwrap.json").exists()
            {
                return Some((dir.to_owned(), PackageManager::Npm));
            }

            current_dir = dir.parent();
        }

        None
    }

    pub fn is_workspaces_enabled(&self) -> bool {
        !self.package_globs.is_empty() && !self.packages.is_empty()
    }

    pub fn load_workspace_packages(&mut self) -> Result<(), PackageGraphError> {
        if self.package_globs.is_empty() {
            return Ok(());
        }

        let mut packages = BTreeMap::new();
        let mut index = 1; // Root is 0

        let mut dirs = glob::walk(&self.root, &self.package_globs)?;
        dirs.sort();

        for dir in dirs {
            if !dir.is_dir() {
                continue;
            }

            let manifest_file = dir.join("package.json");

            if manifest_file.exists() {
                let manifest: PackageJson = json::read_file(manifest_file)?;

                let mut package = Package::new(dir, manifest);
                package.index = index;

                packages.insert(package.get_name()?.to_owned(), package);
                index += 1;
            }
        }

        self.packages = packages;

        Ok(())
    }

    pub fn generate_graph(&mut self) -> Result<(), PackageGraphError> {
        let mut graph = DiGraph::new();

        // Name is optional for the workspace root
        graph.add_node(if self.is_workspaces_enabled() {
            self.root_package
                .manifest
                .name
                .as_deref()
                .unwrap_or("(root)")
                .to_owned()
        } else {
            self.root_package.get_name()?.to_owned()
        });

        if self.package_globs.is_empty() {
            self.graph = graph;

            return Ok(());
        }

        // First pass, create nodes
        {
            for package in self.packages.values_mut() {
                package.node_index = graph.add_node(package.get_name()?.to_owned());
            }
        }

        // Second pass, connect edges
        let add_edge_via_path = |graph: &mut PackageGraphType,
                                 package: &Package,
                                 dep_package: &Package,
                                 path: &Path,
                                 dep_type: DependencyType| {
            if path.is_absolute() && path == dep_package.root
                || path.is_relative() && package.root.join(path).clean() == dep_package.root
            {
                graph.add_edge(package.node_index, dep_package.node_index, dep_type);
            }
        };

        let add_edge_via_version = |graph: &mut PackageGraphType,
                                    package: &Package,
                                    dep_package: &Package,
                                    version: Option<&Version>,
                                    dep_type: DependencyType| {
            if version.is_none() || version == dep_package.manifest.version.as_ref() {
                graph.add_edge(package.node_index, dep_package.node_index, dep_type);
            }
        };

        let add_edges = |graph: &mut PackageGraphType,
                         package: &Package,
                         deps: &DependenciesMap,
                         dep_type: DependencyType| {
            for (name, version) in deps {
                match version {
                    // npm
                    VersionProtocol::Requirement(req) => {
                        if let Some(dep_package) = self.packages.get(name) {
                            if
                            // *
                            req.comparators.is_empty()
                                // ~, ^, etc
                                || dep_package
                                    .manifest
                                    .version
                                    .as_ref()
                                    .is_some_and(|ver| req.matches(ver))
                            {
                                graph.add_edge(
                                    package.node_index,
                                    dep_package.node_index,
                                    dep_type,
                                );
                            }
                        }
                    }
                    VersionProtocol::Version(ver) => {
                        if let Some(dep_package) = self.packages.get(name) {
                            add_edge_via_version(graph, package, dep_package, Some(ver), dep_type);
                        }
                    }
                    // pnpm, yarn
                    VersionProtocol::File(path)
                    | VersionProtocol::Link(path)
                    | VersionProtocol::Portal(path) => {
                        if let Some(dep_package) = self.packages.get(name) {
                            add_edge_via_path(graph, package, dep_package, path, dep_type);
                        }
                    }
                    VersionProtocol::Workspace(ws) => {
                        let (alias, version) = match ws {
                            WorkspaceProtocol::Any { alias } => (alias, &None),
                            WorkspaceProtocol::Tilde { alias, version } => (alias, version),
                            WorkspaceProtocol::Caret { alias, version } => (alias, version),
                            WorkspaceProtocol::File(path) => {
                                if let Some(dep_package) = self.packages.get(name) {
                                    add_edge_via_path(graph, package, dep_package, path, dep_type);
                                }

                                continue;
                            }
                            WorkspaceProtocol::Version(ver) => {
                                if let Some(dep_package) = self.packages.get(name) {
                                    add_edge_via_version(
                                        graph,
                                        package,
                                        dep_package,
                                        Some(ver),
                                        dep_type,
                                    );
                                }

                                continue;
                            }
                        };

                        if let Some(dep_package) =
                            self.packages.get(alias.as_deref().unwrap_or(name))
                        {
                            add_edge_via_version(
                                graph,
                                package,
                                dep_package,
                                version.as_ref(),
                                dep_type,
                            );
                        }
                    }
                    _ => {}
                };
            }
        };

        let mut packages = vec![&self.root_package];
        packages.extend(self.packages.values());

        for package in packages {
            if let Some(deps) = &package.manifest.dependencies {
                add_edges(&mut graph, package, deps, DependencyType::Production);
            }

            if let Some(deps) = &package.manifest.dev_dependencies {
                add_edges(&mut graph, package, deps, DependencyType::Development);
            }

            if let Some(deps) = &package.manifest.peer_dependencies {
                add_edges(&mut graph, package, deps, DependencyType::Peer);
            }

            if let Some(deps) = &package.manifest.optional_dependencies {
                add_edges(&mut graph, package, deps, DependencyType::Optional);
            }
        }

        self.graph = graph;

        Ok(())
    }

    pub fn dependencies_of(
        &self,
        name: &str,
    ) -> Result<Vec<(String, DependencyType)>, PackageGraphError> {
        let package = self
            .packages
            .get(name)
            .ok_or_else(|| PackageGraphError::UnknownPackage(name.to_owned()))?;

        let deps = self
            .graph
            .edges_directed(package.node_index, Direction::Outgoing)
            .map(|edge| {
                (
                    self.graph.node_weight(edge.target()).unwrap().to_owned(),
                    edge.weight().to_owned(),
                )
            })
            .collect();

        Ok(deps)
    }

    pub fn dependents_of(
        &self,
        name: &str,
    ) -> Result<Vec<(String, DependencyType)>, PackageGraphError> {
        let package = self
            .packages
            .get(name)
            .ok_or_else(|| PackageGraphError::UnknownPackage(name.to_owned()))?;

        let deps = self
            .graph
            .edges_directed(package.node_index, Direction::Incoming)
            .map(|edge| {
                (
                    self.graph.node_weight(edge.target()).unwrap().to_owned(),
                    edge.weight().to_owned(),
                )
            })
            .collect();

        Ok(deps)
    }

    pub fn to_dot(&self) -> String {
        format!("{:?}", petgraph::dot::Dot::new(&self.graph))
    }
}
