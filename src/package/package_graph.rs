use super::package::{DependencyType, Package};
use super::package_json::{DependenciesMap, PackageJson, Workspaces};
use super::package_manager::PackageManager;
use super::pnpm_configs::PnpmWorkspace;
use super::version_protocol::VersionProtocol;
use super::workspace_protocol::WorkspaceProtocol;
use petgraph::graph::DiGraph;
use starbase_utils::glob;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub struct PackageGraph {
    cwd: PathBuf,
    graph: DiGraph<u8, DependencyType>,
    manager: PackageManager,
    package_globs: Vec<String>,
    packages: BTreeMap<String, Package>,
    root: PathBuf,
    root_package: Package,
}

impl PackageGraph {
    pub fn load_from<T: AsRef<Path>>(working_dir: T) -> miette::Result<PackageGraph> {
        let working_dir = working_dir.as_ref();

        // Find the root package.json
        let (root, package_manager) = Self::find_package_root(working_dir)
            .unwrap_or_else(|| (working_dir.to_owned(), PackageManager::Npm));

        let root_manifest = PackageJson::load(root.join("package.json"))?;

        // Extract workspaces globs
        let mut package_globs = vec![];

        if package_manager == PackageManager::Pnpm {
            let ws_file = root.join("pnpm-workspace.yaml");

            if ws_file.exists() {
                package_globs = PnpmWorkspace::load(ws_file)?.packages;
            }
        } else if let Some(workspaces) = &root_manifest.workspaces {
            package_globs = match workspaces {
                Workspaces::Globs(globs) => globs.to_owned(),
                Workspaces::Config { packages, .. } => packages.to_owned(),
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
            // pnpm
            if dir.join("pnpm-lock.yaml").exists() {
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

    pub fn load_workspace_packages(&mut self) -> miette::Result<()> {
        if self.package_globs.is_empty() {
            return Ok(());
        }

        let mut packages = BTreeMap::new();
        let mut index = 1; // Root is 0

        for dir in glob::walk(&self.root, &self.package_globs)? {
            if !dir.is_dir() {
                continue;
            }

            let manifest_file = dir.join("package.json");

            if manifest_file.exists() {
                let manifest = PackageJson::load(manifest_file)?;
                let name = manifest.name.clone();

                let mut package = Package::new(dir, manifest);
                package.index = index;

                packages.insert(name, package);
                index += 1;
            }
        }

        self.packages = packages;

        Ok(())
    }

    pub fn generate_graph(&mut self) -> miette::Result<()> {
        self.graph.add_node(self.root_package.index);

        if self.package_globs.is_empty() {
            return Ok(());
        }

        // First pass, create nodes
        {
            for package in self.packages.values_mut() {
                package.node_index = self.graph.add_node(package.index);
            }
        }

        // Second pass, connect edges
        let mut add_edges =
            |package: &Package, deps: &DependenciesMap, dep_type: DependencyType| {
                for (name, version) in deps {
                    match version {
                        VersionProtocol::File(path)
                        | VersionProtocol::Link(path)
                        | VersionProtocol::Portal(path) => {
                            if let Some(dep_package) = self.packages.get(name) {
                                if package.root.join(path) != dep_package.root {
                                    self.graph.add_edge(
                                        package.node_index,
                                        dep_package.node_index,
                                        dep_type,
                                    );
                                }
                            }
                        }
                        VersionProtocol::Workspace(ws) => {
                            let (alias, version) = match ws {
                                WorkspaceProtocol::Any { alias } => (alias, &None),
                                WorkspaceProtocol::Tilde { alias, version } => (alias, version),
                                WorkspaceProtocol::Caret { alias, version } => (alias, version),
                                WorkspaceProtocol::File(path) => {
                                    if let Some(dep_package) = self.packages.get(name) {
                                        if package.root.join(path) != dep_package.root {
                                            self.graph.add_edge(
                                                package.node_index,
                                                dep_package.node_index,
                                                dep_type,
                                            );
                                        }
                                    }

                                    continue;
                                }
                                WorkspaceProtocol::Version(ver) => {
                                    if let Some(dep_package) = self.packages.get(name) {
                                        if dep_package
                                            .manifest
                                            .version
                                            .as_ref()
                                            .is_some_and(|v| v == ver)
                                        {
                                            self.graph.add_edge(
                                                package.node_index,
                                                dep_package.node_index,
                                                dep_type,
                                            );
                                        }
                                    }

                                    continue;
                                }
                            };

                            if let Some(dep_package) =
                                self.packages.get(alias.as_deref().unwrap_or(name))
                            {
                                if version.is_none()
                                    || version.as_ref() == dep_package.manifest.version.as_ref()
                                {
                                    self.graph.add_edge(
                                        package.node_index,
                                        dep_package.node_index,
                                        dep_type,
                                    );
                                }
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
                add_edges(package, deps, DependencyType::Production);
            }

            if let Some(deps) = &package.manifest.dev_dependencies {
                add_edges(package, deps, DependencyType::Development);
            }

            if let Some(deps) = &package.manifest.peer_dependencies {
                add_edges(package, deps, DependencyType::Peer);
            }
        }

        Ok(())
    }
}
