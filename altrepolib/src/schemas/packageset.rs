use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RepositoryStatisticsResponse {
    length: usize,
    branches: Vec<BranchInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BranchInfo {
    branch: String,
    date_update: String,
    packages_count: Vec<PackagesCountInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackagesCountInfo {
    arch: String,
    component: String,
    count: usize,
    size: usize,
    size_hr: String,
    uuid: String,
}