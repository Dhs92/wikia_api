extern crate reqwest;
extern crate serde;
extern crate serde_json;

use super::page::Page;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Results {
    batches: i32,
    items: Vec<Page>,
    total: i32,
    currentBatch: i32,
    next: i32,
}

impl Results {
}

