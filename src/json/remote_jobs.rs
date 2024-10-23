use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Website {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RemoteJobCategory {
    pub category: String,
    pub sites: Vec<Website>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RemoteJobs {
    pub remote_sites: Vec<RemoteJobCategory>,
}
