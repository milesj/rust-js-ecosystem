use crate::compiler_options::CompilerOptions;
use crate::path_types::PathOrGlob;
use serde::Deserialize;
use std::collections::{BTreeMap, VecDeque};
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TsConfigExtends {
    Single(PathBuf),
    Multiple(Vec<PathBuf>),
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TsConfigJson {
    pub compile_on_save: Option<bool>,

    pub compiler_options: Option<CompilerOptions>,

    pub exclude: Option<Vec<PathOrGlob>>,

    pub extends: Option<TsConfigExtends>,

    pub files: Option<Vec<PathBuf>>,

    pub include: Option<Vec<PathOrGlob>>,

    pub references: Option<Vec<ProjectReference>>,

    pub type_acquisition: Option<TypeAcquisition>,

    #[serde(flatten)]
    pub other_fields: BTreeMap<String, serde_json::Value>,
}

pub struct ResolvedTsConfigChain {
    pub path: PathBuf,
    pub config: TsConfigJson,
}

impl TsConfigJson {
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

                for extends_path in match extends {
                    TsConfigExtends::Single(other) => vec![other],
                    TsConfigExtends::Multiple(others) => others.iter().collect(),
                } {
                    queue.push_back(if extends_path.extension().is_none() {
                        parent_dir.join(extends_path).join("tsconfig.json")
                    } else {
                        parent_dir.join(extends_path)
                    })
                }
            }

            chain.push(ResolvedTsConfigChain { path, config })
        }

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
