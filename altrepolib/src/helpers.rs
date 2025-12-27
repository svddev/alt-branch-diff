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
        let b1_packages = get_branch_binary_packages(branch_1, Some(arch.as_str()))
            .await?
            .packages
            .into_iter()
            .collect::<HashSet<PackageInfo>>();

        let b1_packages_names = b1_packages.clone().into_iter().map(|el| {el.name}).collect::<HashSet<String>>();

        let b2_packages = get_branch_binary_packages(branch_2, Some(arch.as_str()))
            .await?
            .packages
            .into_iter()
            .collect::<HashSet<PackageInfo>>();

        let b2_packages_names = b2_packages.clone().into_iter().map(|el| {el.name}).collect::<HashSet<String>>();


        let a_names = b1_packages_names
            .difference(&b2_packages_names)
            .cloned()
            .collect::<HashSet<String>>();
        
        let a_packages = b1_packages
            .clone()
            .into_iter()
            .filter(|el| {a_names.contains(&el.name)})
            .collect::<Vec<PackageInfo>>();

        let b_names = b2_packages_names
            .difference(&b1_packages_names)
            .cloned()
            .collect::<HashSet<String>>();
        
        let b_packages = b2_packages
            .clone()
            .into_iter()
            .filter(|el| {b_names.contains(&el.name)})
            .collect::<Vec<PackageInfo>>();

        let c: Vec<PackageInfo> = b1_packages
            .into_iter()
            .zip(b2_packages)
            .filter(|(left, right)| { left.name == right.name && left.version > right.version })
            .map(|el| { el.0 })
            .collect();

        result.insert(arch, DiffResultArch { additional_packages: a_packages, missing_packages: b_packages, version_greater: c });
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