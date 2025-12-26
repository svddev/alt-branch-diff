use crate::api::{export::get_branch_binary_packages, packageset::get_repository_statistics};
use crate::constants::ARCHES;
use crate::schemas::{DiffResult, DiffResultArch, PackageInfo};
use std::collections::{HashMap, HashSet};

pub async fn get_branch_diff(branch_1: &str, branch_2: &str) -> anyhow::Result<DiffResult> {

    let mut result = DiffResult::new();
    
    for arch in ARCHES {
   
        let b1_packages = get_branch_binary_packages(branch_1, Some(arch))
            .await?
            .packages
            .into_iter()
            .collect::<HashSet<PackageInfo>>();

        let b2_packages = get_branch_binary_packages(branch_2, Some(arch))
            .await?
            .packages
            .into_iter()
            .collect::<HashSet<PackageInfo>>();

        let a: Vec<PackageInfo> = b1_packages
            .difference(&b2_packages)
            .into_iter()
            .cloned()
            .collect();
        let b: Vec<PackageInfo> = b2_packages
            .difference(&b1_packages)
            .into_iter()
            .cloned()
            .collect();

        let c: Vec<PackageInfo> = b1_packages
            .into_iter()
            .zip(b2_packages)
            .filter(|(left, right)| { left.name == right.name && left.version > right.version })
            .map(|el| { el.0 })
            .collect();

        result.insert(arch.to_string(), DiffResultArch { additional_packages: a, missing_packages: b, version_greater: c });
    }

    Ok(result)
}

pub async fn get_list_of_branches() -> anyhow::Result<Vec<String>> {
    let res = get_repository_statistics(None).await?;
    let branches: Vec<String> = res
        .branches.clone()
        .into_iter()
        .map(|el | {el.branch}).collect();
    Ok(branches)
}

pub async fn get_list_of_architectures(branch: &str) -> anyhow::Result<HashSet<String>> {
    let res = get_repository_statistics(Some(branch)).await?;
    let arches  = res
        .branches[0].clone()
        .packages_count
        .into_iter()
        .map(|el | {el.arch}).collect();
    Ok(arches)
}

pub async fn get_full_list_of_architectures() -> anyhow::Result<HashMap<String, HashSet<String>>> {
    let res = get_repository_statistics(None).await?;
    let mut result: HashMap<String, HashSet<String>> = HashMap::new();
    for branch in res.branches.clone() {
        let arches: HashSet<String> = branch.packages_count
            .into_iter()
            .map(|el| {el.arch})
            .collect();

        result.insert(branch.branch, arches);
    }

    Ok(result)
}