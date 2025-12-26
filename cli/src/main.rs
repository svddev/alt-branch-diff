use std::collections::HashSet;

// use altrepolib::api::export::get_branch_binary_packages;
// use altrepolib::api::packageset::get_repository_statistics;
use altrepolib::helpers::{get_full_list_of_architectures, get_list_of_architectures, get_list_of_branches};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let res = get_full_list_of_architectures().await?;
    for el in res.into_iter() {
        println!("{}: {:?}", el.0.to_uppercase(), el.1);
    }
    Ok(())
}
