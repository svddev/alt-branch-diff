use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RepositoryStatisticsResponse {
    pub length: usize,
    pub branches: Vec<BranchInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BranchInfo {
    pub branch: String,
    pub date_update: String,
    pub packages_count: Vec<PackagesCountInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PackagesCountInfo {
    pub arch: String,
    pub component: String,
    pub count: usize,
    pub size: usize,
    pub size_hr: String,
    pub uuid: String,
}