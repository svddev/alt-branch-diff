use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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

impl DiffResultArch {
    pub fn new(b1_packages: (HashSet<PackageInfo>, HashSet<String>), b2_packages: (HashSet<PackageInfo>, HashSet<String>)) -> Self {
        let a_names = b1_packages.1
            .difference(&b2_packages.1)
            .cloned()
            .collect::<HashSet<String>>();
        
        let a_packages = b1_packages.0
            .clone()
            .into_iter()
            .filter(|el| {a_names.contains(&el.name)})
            .collect::<Vec<PackageInfo>>();

        let b_names = b2_packages.1
            .difference(&b1_packages.1)
            .cloned()
            .collect::<HashSet<String>>();
        
        let b_packages = b2_packages.0
            .clone()
            .into_iter()
            .filter(|el| {b_names.contains(&el.name)})
            .collect::<Vec<PackageInfo>>();

        let c: Vec<PackageInfo> = b1_packages.0
            .into_iter()
            .zip(b2_packages.0)
            .filter(|(left, right)| { left.name == right.name && left.version > right.version })
            .map(|el| { el.0 })
            .collect();

        Self {
            additional_packages: a_packages,
            missing_packages: b_packages,
            version_greater: c,
        }
    }
}