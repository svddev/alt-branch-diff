use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::schemas::PackageInfo;


#[derive(Debug, Deserialize, Serialize)]
pub struct BBPResponse {
    pub request_args: HashMap<String, String>,
    pub length: usize,
    pub packages: Vec<PackageInfo>
}