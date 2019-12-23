use crate::error::KyaniteError;
use clap::{crate_authors, crate_description, crate_name, crate_version, Arg};

#[derive(Clone, Debug)]
pub struct KyaniteParams {
    pub verbose: bool,
    pub debug: bool,
    pub sources: Vec<String>,
    pub tags: Vec<String>,
    pub insane: bool,
}

impl KyaniteParams {
    pub fn new() -> Result<Self, KyaniteError> {
        let matches =
            clap::app_from_crate!()
                .arg(
                    Arg::with_name("debug").long("debug").short("d").help(
                        "Goes through the collection processing without downloading anything",
                    ),
                )
                .arg(
                    Arg::with_name("verbose")
                        .long("verbose")
                        .short("v")
                        .help("Display debug logs"),
                )
                .arg(
                    Arg::with_name("sources")
                        .long("sources")
                        .short("s")
                        .help("The website to scrap. Type \"all\" for all, separate multiple with a comma.").
                        value_name("sources")
                        .default_value("all"))
                .arg(
                    Arg::with_name("tags")
                        .long("tags")
                        .short("t")
                        .help("Define the tags you wish to scrap, separate multiple with a comma")
                        .value_name("tags"),
                )
                .arg(
                    Arg::with_name("insanity")
                        .long("insanity")
                        .short("i")
                        .help("Overrides the empty tag limitation, allowing you to scrap entire websites.")
                )
                .get_matches();
        let verbose = matches.is_present("verbose");
        let debug = matches.is_present("debug");
        let insane = matches.is_present("insanity");
        let sources = match matches.value_of("sources") {
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
        let tags = match matches.value_of("tags") {
            Some(tags) => {
                let mut clean = Vec::<String>::new();
                let pieces = tags.split(",");
                for piece in pieces {
                    clean.push(piece.trim().to_owned())
                }
                clean.sort();
                clean
            }
            None => Vec::new(),
        };
        Ok(Self {
            verbose,
            debug,
            sources,
            tags,
            insane,
        })
    }
}
