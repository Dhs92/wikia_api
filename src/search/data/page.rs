extern crate serde_json;

use serde_json::Value;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Page {
    id: i32,
    title: String,
    url: String,
    #[serde(alias="ns")]
    namespace: i8,
    quality: i8,
    snippet: String
}

impl Page {
    //pub fn from_string(page: &str) -> Result<Self, ()>
    //{
    //   Ok(()) 
    //} 
}
