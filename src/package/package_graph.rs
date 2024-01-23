use super::package::Package;
use super::package_json::{PackageJson, Workspaces};
use super::package_manager::PackageManager;
use super::pnpm_configs::PnpmWorkspace;
use starbase_utils::glob;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub struct PackageGraph {
    cwd: PathBuf,
    manager: PackageManager,
    monorepo: bool,
    package_globs: Vec<String>,
    packages: BTreeMap<String, Package>,
    root: PathBuf,
    root_package: Package,
}

impl PackageGraph {
    pub fn load_from<T: AsRef<Path>>(working_dir: T) -> miette::Result<PackageGraph> {
        let working_dir = working_dir.as_ref();
        let (root, package_manager) = Self::find_package_root(working_dir);
        let root_manifest = PackageJson::load(root.join("package.json"))?;
        let mut package_globs = vec![];

        // Extract workspaces globs
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
            manager: package_manager,
            monorepo: !package_globs.is_empty(),
            package_globs,
            packages: BTreeMap::new(),
            root_package: Package {
                manifest: root_manifest,
                root: root.clone(),
            },
            root,
        })
    }

    pub fn find_package_root<T: AsRef<Path>>(starting_dir: T) -> (PathBuf, PackageManager) {
        let starting_dir = starting_dir.as_ref();
        let mut current_dir = Some(starting_dir);

        while let Some(dir) = current_dir {
            // pnpm
            if dir.join("pnpm-lock.yaml").exists() {
                return (dir.to_owned(), PackageManager::Pnpm);
            }
            // yarn
            else if dir.join("yarn.lock").exists() {
                return (
                    dir.to_owned(),
                    if dir.join(".yarn").exists() || dir.join(".yarnrc.yml").exists() {
                        PackageManager::Yarn
                    } else {
                        PackageManager::YarnLegacy
                    },
                );
            }
            // npm
            else if dir.join("package-lock.json").exists()
                || dir.join("npm-shrinkwrap.json").exists()
            {
                return (dir.to_owned(), PackageManager::Npm);
            }

            current_dir = dir.parent();
        }

        // Nothing, assume the working directory
        (starting_dir.to_owned(), PackageManager::Npm)
    }

    fn load_packages(&mut self) -> miette::Result<()> {
        if !self.monorepo {
            return Ok(());
        }

        let mut packages = BTreeMap::new();

        for dir in glob::walk(&self.root, &self.package_globs)? {
            let manifest_file = dir.join("package.json");

            if manifest_file.exists() {
                let manifest = PackageJson::load(manifest_file)?;

                packages.insert(
                    manifest.name.clone(),
                    Package {
                        manifest,
                        root: dir,
                    },
                );
            }
        }

        self.packages = packages;

        Ok(())
    }
}
