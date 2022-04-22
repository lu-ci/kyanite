use clap::{command, Arg};

#[derive(Clone, Debug)]
pub struct KyaniteParams {
    pub verbose: bool,
    pub debug: bool,
    pub sources: Vec<String>,
    pub tags: Vec<String>,
    pub insane: bool,
}

impl KyaniteParams {
    pub fn new() -> anyhow::Result<Self> {
        let matches = command!()
                .arg(
                    Arg::new("debug").long("debug").short('d').help(
                        "Goes through the collection processing without downloading anything",
                    ),
                )
                .arg(
                    Arg::new("verbose")
                        .long("verbose")
                        .short('v')
                        .help("Display debug logs"),
                )
                .arg(
                    Arg::new("sources")
                        .long("sources")
                        .short('s')
                        .help("The website to scrap. Type \"all\" for all, separate multiple with a comma.").
                        value_name("sources")
                        .default_value("all"))
                .arg(
                    Arg::new("tags")
                        .long("tags")
                        .short('t')
                        .help("Define the tags you wish to scrap, separate multiple with a comma")
                        .value_name("tags"),
                )
                .arg(
                    Arg::new("insanity")
                        .long("insanity")
                        .short('i')
                        .help("Overrides the empty tag limitation, allowing you to scrap entire websites.")
                )
                .get_matches();
        let verbose = matches.is_present("verbose");
        let debug = matches.is_present("debug");
        let insane = matches.is_present("insanity");
        let sources = match matches.value_of("sources") {
            Some(srcs) => {
                let mut clean = Vec::<String>::new();
                let pieces = srcs.split(',');
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
                let pieces = tags.split(',');
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
