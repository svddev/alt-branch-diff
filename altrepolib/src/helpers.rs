use crate::api::{export::get_branch_binary_packages, packageset::get_repository_statistics};
use crate::constants::PSEUDO_ARCHES;
use crate::schemas::{DiffResult, DiffResultArch, PackageInfo};

use std::collections::{HashMap, HashSet};

pub async fn get_branch_diff(branch_1: &str, branch_2: &str) -> anyhow::Result<DiffResult> {
    let mut result = DiffResult::new();
    
    let b1_arches = get_list_of_architectures(branch_1).await?;
    let b2_arches = get_list_of_architectures(branch_2).await?;

    let arches = b1_arches
        .intersection(&b2_arches)
        .filter(|el| { !PSEUDO_ARCHES.contains(&el.as_str()) })
        .cloned()
        .collect::<Vec<String>>();

    for arch in arches {   
        let b1_packages=  get_packages(branch_1, arch.as_str()).await?;       
        let b2_packages = get_packages(branch_2, arch.as_str()).await?; 
        let diff_result = DiffResultArch::new(b1_packages, b2_packages);
        result.insert(arch, diff_result);
    }

    Ok(result)
}

pub async fn get_packages(branch: &str, arch: &str) -> anyhow::Result<(HashSet<PackageInfo>, HashSet<String>)> {
    let packages = get_branch_binary_packages(branch, Some(arch))
            .await?
            .packages
            .into_iter()
            .collect::<HashSet<PackageInfo>>();
    let names = packages.clone().into_iter().map(|el| {el.name}).collect::<HashSet<String>>();
    Ok((packages, names))
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