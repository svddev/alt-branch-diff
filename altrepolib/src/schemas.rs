use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod api;

#[derive(Clone, Debug, Eq, Deserialize, Serialize, Hash)]
pub struct PackageInfo {
    pub name: String,
    pub epoch: usize,
    pub version: String,
    pub release: String,
    pub arch: String,
    pub disttag: String,
    pub buildtime: usize,
    pub source: String,
}

impl PartialEq for PackageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}



pub type DiffResult = HashMap<String, DiffResultArch>;

#[derive(Debug, Deserialize, Serialize)]
pub struct DiffResultArch {
    pub additional_packages: Vec<PackageInfo>,
    pub missing_packages: Vec<PackageInfo>,
    pub version_greater: Vec<PackageInfo>
}