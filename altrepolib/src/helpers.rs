use std::{collections::HashMap, hash::Hash};

use crate::api::packageset::get_repository_statistics;

pub async fn get_list_of_branches() -> anyhow::Result<Vec<String>> {
    let res = get_repository_statistics(None).await?;
    let branches: Vec<String> = res.branches.clone().into_iter().map(|el | {el.branch}).collect();
    Ok(branches)
}

pub async fn get_list_of_architectures(branch: &str) -> anyhow::Result<Vec<String>> {
    let res = get_repository_statistics(Some(branch)).await?;
    let arches: Vec<String> = res.branches[0].clone().packages_count.into_iter().map(|el | {el.arch}).collect();
    Ok(arches)
}

pub async fn get_full_list_of_architectures() -> anyhow::Result<HashMap<String, Vec<String>>> {
    let res = get_repository_statistics(None).await?;

    let mut result: HashMap<String, Vec<String>> = HashMap::new();

    for branch in res.branches.clone() {
        let arches: Vec<String> = branch.packages_count
            .into_iter()
            .map(|el| {el.arch})
            .collect();

        result.insert(branch.branch, arches);
    }

    Ok(result)
}