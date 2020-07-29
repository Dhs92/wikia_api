#![allow(unused_imports)]
mod search;
pub use search::Search;

mod results;
pub use results::Results;

mod error;
pub use error::Error;

mod page;
use page::Page;