use crate::compiler_options::CompilerOptions;
use crate::path_types::PathOrGlob;
use serde::Deserialize;
use std::collections::{BTreeMap, VecDeque};
use std::path::{Path, PathBuf};
use std::{fs, io};

// https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-0.html#supporting-multiple-configuration-files-in-extends
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ExtendsField {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TsConfigJson {
    pub compile_on_save: Option<bool>,

    pub compiler_options: Option<CompilerOptions>,

    pub exclude: Option<Vec<PathOrGlob>>,

    pub extends: Option<ExtendsField>,

    pub files: Option<Vec<PathBuf>>,

    pub include: Option<Vec<PathOrGlob>>,

    pub references: Option<Vec<ProjectReference>>,

    pub type_acquisition: Option<TypeAcquisition>,

    #[serde(flatten)]
    pub other_fields: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, PartialEq)]
pub struct ResolvedTsConfigChain {
    pub path: PathBuf,
    pub config: TsConfigJson,
}

impl TsConfigJson {
    pub fn resolve_path_in_node_modules<N: AsRef<str>, D: AsRef<Path>>(
        package_file: N,
        starting_dir: D,
    ) -> Option<PathBuf> {
        let package_file = package_file.as_ref();
        let mut current_dir = Some(starting_dir.as_ref());

        while let Some(dir) = current_dir {
            let file_path = if package_file.ends_with(".json") {
                dir.join("node_modules").join(package_file)
            } else {
                dir.join("node_modules")
                    .join(package_file)
                    .join("tsconfig.json")
            };

            if file_path.exists() {
                return Some(file_path);
            }

            current_dir = dir.parent();
        }

        None
    }

    pub fn resolve_extends_chain<T: AsRef<Path>>(
        path: T,
    ) -> io::Result<Vec<ResolvedTsConfigChain>> {
        let mut chain = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(path.as_ref().to_owned());

        while let Some(path) = queue.pop_front() {
            let config = fs::read(&path)?;
            let config: TsConfigJson = serde_json::from_slice(&config)?;

            if let Some(extends) = &config.extends {
                let parent_dir = path.parent().unwrap();

                for extends_from in match extends {
                    ExtendsField::Single(other) => vec![other],
                    ExtendsField::Multiple(others) => others.iter().rev().collect(),
                } {
                    // File path
                    if extends_from.starts_with('.') {
                        queue.push_back(if extends_from.ends_with(".json") {
                            parent_dir.join(extends_from)
                        } else {
                            parent_dir.join(extends_from).join("tsconfig.json")
                        });
                    }
                    // Node module
                    else if let Some(package_path) =
                        Self::resolve_path_in_node_modules(extends_from, parent_dir)
                    {
                        queue.push_back(package_path);
                    }
                }
            }

            chain.push(ResolvedTsConfigChain { path, config })
        }

        // Reverse so that the base file is the 0-index,
        // and the files that overwrite it come next
        chain.reverse();

        Ok(chain)
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct ProjectReference {
    pub path: PathBuf,
    pub prepend: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypeAcquisition {
    pub enable: bool,

    pub include: Option<Vec<String>>,

    pub exclude: Option<Vec<String>>,

    pub disable_filename_based_type_acquisition: Option<bool>,
}
