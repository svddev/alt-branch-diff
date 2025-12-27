use std::collections::{HashMap, HashSet};

// use altrepolib::api::export::get_branch_binary_packages;
// use altrepolib::api::packageset::get_repository_statistics;
use altrepolib::helpers::{get_branch_diff, get_full_list_of_architectures, get_list_of_architectures, get_list_of_branches};

const BRANCH_1: &str = "sisyphus";
const BRANCH_2: &str = "p11";

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let res = get_branch_diff(BRANCH_1, BRANCH_2).await?;

    println!("{}", serde_json::to_string(&res)?);
    
    Ok(())
}
