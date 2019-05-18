extern crate flexi_logger;
extern crate log;
extern crate wikia_api;

use flexi_logger::{opt_format, Logger};
use wikia_api::search::search::Search;

#[test]
fn search() -> Result<(), Box<std::error::Error>> {
    let search = Search::new()
        .sub_wikia("thedivision")
        .query("M4")
        .search()?;

    Logger::with_env_or_str("info")
        .log_to_file()
        .directory("log_files")
        .format(opt_format)
        .start()?;

    log::info!("{:?}", search);

    Ok(())
}
