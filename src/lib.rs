#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use url::Url;

#[derive(Debug, StructOpt)]
#[structopt(name = "offdocs", about = "offdocs is a multi-source static documentation generator")]
struct Cli {
    #[structopt(short = "b", long = "base")]
    base_angular_repo: String,

    /// If `base_angular_repo` is empty or does not exist, then create it from this
    template: Option<Url>,

    config: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    wiki: Option<DownloadTargeted>,

    code: Option<DownloadTargeted>,

    blog: Option<DownloadTargeted>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DownloadTargeted {
    /// Download URL (can be a git repository or archive)
    url: Url,
    /// Target directory
    target: String,
    /// Command to run before downloading
    before: Option<Vec<String>>,
    /// Command to run after downloading
    after: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn it_works() {
        let config = Cli {
            base_angular_repo: env::current_dir()
                .unwrap()
                .join("angular")
                .into_os_string()
                .into_string()
                .unwrap(),
            template: None,
            config: String::from("")
        };
    }
}
