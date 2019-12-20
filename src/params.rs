use crate::error::KyaniteError;
use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches};

pub struct KyaniteParams {
    pub verbose: bool,
    pub test: bool,
    pub sources: Vec<String>,
}

impl KyaniteParams {
    pub fn new() -> Result<Self, KyaniteError> {
        let matches: ArgMatches =
            clap::app_from_crate!()
                .arg(
                    Arg::with_name("test").long("test").short("t").help(
                        "Goes through the collection processing without downloading anything",
                    ),
                )
                .arg(
                    Arg::with_name("verbose")
                        .long("verbose")
                        .short("v")
                        .help("Display debug logs"),
                )
                .arg(Arg::with_name("source").long("source").short("s").help(
                    "The website to scrap. Type \"all\" for all, separate multiple with a comma",
                ))
                .get_matches();
        let verbose = matches.is_present("verbose");
        let test = matches.is_present("test");
        let sources = match matches.value_of("source") {
            Some(srcs) => {
                let mut clean = Vec::<String>::new();
                let pieces = srcs.split(",");
                for piece in pieces {
                    clean.push(piece.trim().to_owned())
                }
                clean
            }
            None => Vec::new(),
        };
        Ok(Self {
            verbose,
            test,
            sources,
        })
    }
}
