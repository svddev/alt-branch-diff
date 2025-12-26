use std::collections::HashSet;

use altrepolib::api::export::get_branch_binary_packages;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bbp_res = get_branch_binary_packages("p11", Some("e2k")).await?;
    let arches: HashSet<String> = bbp_res.packages.into_iter().map(|el| { el.arch }).collect();
    println!("Archetectures: {:?}", arches);
    
    Ok(())
}
