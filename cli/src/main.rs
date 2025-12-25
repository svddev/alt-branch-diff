use altrepolib::api::get_branch_binary_packages;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bbp_res = get_branch_binary_packages("p11", "x86_64").await?;
    //println!("Body:\n{:?}", resp);
    for it in bbp_res.packages {
        println!("{}", it.name)
    }
    Ok(())
}
