extern crate reqwest;
extern crate serde_json;
extern crate url;

use super::results::Results;
use std::option::Option;
use url::Url;

#[derive(Debug)]
#[allow(missing_docs)]
pub struct Search {
    query: Option<String>,
    sub_wikia: Option<String>,
    namespace: Option<String>,
    limits: Option<String>,
}

impl Default for Search {
    fn default() -> Self {
        Search {
            query: None,
            sub_wikia: None,
            namespace: None,
            limits: None,
        }
    }
}

/// ## Example
/// ```
/// use wikia_api::search::Results;
/// use wikia_api::search::Search;
///
/// let search = Search::new()
///              .sub_wikia("thedivision")
///              .query("test")
///              .namespace(&[0])
///              .limits(10)
///              .search().unwrap();
///
/// eprintln!("Printing output of search: ");
/// eprintln!("{:?}", search);
///
/// ```
impl Search {
    /// Constructs an empty Search object
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the query for the search
    ///
    /// Note: Can not be empty when submitting search
    pub fn query(&mut self, query: &str) -> &mut Self {
        self.query = Some(query.to_string());

        self
    }

    /// Set the sub wiki for the search
    pub fn sub_wikia(&mut self, sub_wikia: &str) -> &mut Self {
        self.sub_wikia = Some(sub_wikia.to_string());

        self
    }

    /// Set the namespace for the search
    ///
    /// [Namespaces](https://community.fandom.com/wiki/Help:Namespace)
    pub fn namespace(&mut self, namespace: &[u8]) -> &mut Self {
        let mut tmp = String::new();

        for i in namespace {
            tmp += &format!("{},", i);
        }

        tmp.pop();

        self.namespace = Some(tmp);

        self
    }

    /// Set a limit for the number of batches returned
    ///
    /// Default: 5
    pub fn limits(&mut self, limits: u8) -> &mut Self {
        let tmp = format!("{}", limits);

        self.limits = Some(tmp);

        self
    }

    fn build_url(&'a self) -> std::result::Result<reqwest::Url, Box<dyn std::error::Error>> {
        let mut base_url = String::from("http://");

        match self.sub_wikia.as_ref() {
            Some(sub_w) => base_url.push_str(&format!("{}.wikia.com/api/v1/Search/List?", sub_w)),
            None => base_url.push_str("wikia.com/api/v1/Search/List?"),
        };

        let mut query_strings: Vec<(&str, &str)> = Vec::new();

        #[allow(clippy::single_match)] // TODO throw Error if empty
        match self.query.as_ref() {
            Some(query) => query_strings.push(("query", &query)),
            None => (), 
        };

        match self.limits.as_ref() {
            Some(limits) => query_strings.push(("limits", &limits)),
            None => query_strings.push(("limits", "5")),
        };

        match self.namespace.as_ref() {
            Some(ns) => query_strings.push(("namespaces", &ns)),
            None => query_strings.push(("namespaces", "0")),
        }

        let url = Url::parse_with_params(&base_url, query_strings)?;

        Ok(url)
    }

    /// Build the URL from the provided options and submit the search
    pub fn search(&'a self) -> std::result::Result<Results, Box<dyn std::error::Error>> {
        let url = self.build_url()?;

        let result_string = reqwest::get(url)?.text()?;

        let result: Results = serde_json::from_str(&result_string)?;

        Ok(result)
    }
}
