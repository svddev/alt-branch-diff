use crate::schemas::BBPResponse;

pub async fn get_branch_binary_packages(branch: &str, arch: &str) -> anyhow::Result<BBPResponse> {
    Ok(reqwest::Client::new()
        .get(format!("https://rdb.altlinux.org/api/export/branch_binary_packages/{branch}?arch={arch}"))
        .send()
        .await?
        .json()
        .await?)
}
