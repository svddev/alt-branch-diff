use clap::Parser;
use altrepolib::helpers::get_branch_diff;

const BRANCH_1: &str = "sisyphus";
const BRANCH_2: &str = "p11";

#[derive(Parser)]
struct Args {
    branch_1: String,
    branch_2: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();


    let res = get_branch_diff(&args.branch_1, &args.branch_2).await?;

    println!("{}", serde_json::to_string(&res)?);
    
    Ok(())
}
