use serde::{Deserialize, Serialize};

// Define a struct that matches your JSON structure
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: Option<String>,
    pub phones: Vec<String>,
}
