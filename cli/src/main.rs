use std::collections::HashSet;

// use altrepolib::api::export::get_branch_binary_packages;
use altrepolib::api::packageset::get_repository_statistics;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let res = get_repository_statistics(None).await?;
    let branches: Vec<String> = res.branches.clone().into_iter().map(|el | {el.branch}).collect();
    println!("Branches: {:?}", branches);

    for branch in res.branches.clone().into_iter() {
        println!("Branch: {}", branch.branch);

        let arches: HashSet<String> = branch.packages_count.into_iter().map(|el| {el.arch}).collect();
        println!("Arches: {:?}", arches);

    }
    
    Ok(())
}
