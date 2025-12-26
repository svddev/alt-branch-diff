use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct BBPResponse {
    pub request_args: HashMap<String, String>,
    pub length: usize,
    pub packages: Vec<PackageInfo>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageInfo {
    pub name: String,
    pub epoch: usize,
    pub version: String,
    pub release: String,
    pub arch: String,
    pub disttag: String,
    pub buildtime: usize,
    pub source: String,
}