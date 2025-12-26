use std::{str::FromStr, time::Duration};
use reqwest::{Client, Url};
use crate::schemas::api::export::BBPResponse;
use crate::constants::REQUEST_TIMEOUT;

pub async fn get_branch_binary_packages(branch: &str, arch: Option<&str>) -> anyhow::Result<BBPResponse> {

    let url: Url;
    if let Some(arch_str) = arch {
        url = Url::from_str(format!("https://rdb.altlinux.org/api/export/branch_binary_packages/{branch}?arch={arch_str}").as_str())?;
    } else {
        url = Url::from_str(format!("https://rdb.altlinux.org/api/export/branch_binary_packages/{branch}").as_str())?;
    }

    let resp = Client::new()
        .get(url)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT))
        .send()
        .await?
        .error_for_status()?;
    
    Ok(resp.json().await?)
}

