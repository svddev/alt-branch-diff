use std::{str::FromStr, time::Duration};
use reqwest::{Client, Url};

use crate::constants::REQUEST_TIMEOUT;
use crate::schemas::packageset::RepositoryStatisticsResponse;

pub async fn get_repository_statistics(branch: Option<&str>) -> anyhow::Result<RepositoryStatisticsResponse> {
    let url: Url;
    if let Some(branch_str) = branch {
        url = Url::from_str(format!("https://rdb.altlinux.org/api/packageset/repository_statistics?branch={branch_str}").as_str())?;
    } else {
        url = Url::from_str(format!("https://rdb.altlinux.org/api/packageset/repository_statistics").as_str())?;
    }

    let resp = Client::new()
        .get(url)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT))
        .send()
        .await?
        .error_for_status()?;
    
    Ok(resp.json().await?) 
}