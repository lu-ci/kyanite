#[macro_use] extern crate clap;
#[macro_use] extern crate serde_derive;

mod error;
mod utils;
mod modules;
mod collector;
mod downloader;

use clap::{App, Arg};
use self::error::Error;
use self::downloader::Downloader;
use self::collector::{Collector, CollectorSlave};

fn main() -> Result<(), Error> {
    let ap = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(Arg::with_name("INPUT")
            .help("The name of the service you want to rip.")
            .required(true)
            .index(1)
        )
        .arg(Arg::with_name("limit")
            .help("Maximum number of items to grab.")
            .long("limit")
            .short("l")
            .value_name("LIMIT")
        )
        .arg(Arg::with_name("verbosity")
            .help("Sets the verbosity level.")
            .multiple(true)
            .short("v")
        )
        .arg(Arg::with_name("tags")
            .help("Comma separated tags to collect and rip.")
            .long("tags")
            .short("t")
            .value_name("TAGS")
        )
        .get_matches();
    let services: &str = match ap.value_of("INPUT") {
        Some(services) => services,
        None => ""
    };
    let mut tags: Vec<&str> = match ap.value_of("tags") {
        Some(tags) => {
            tags.split(",").collect::<Vec<&str>>()
        },
        None => Vec::new()
    };
    tags.sort();
    let limit: u32 = match ap.value_of("limit") {
        Some(limit) => match limit.parse::<u32>() {
            Ok(limit) => limit,
            Err(_) => 999999999
        },
        None => 999999999
    };
    let _verbosity: u8 = ap.occurrences_of("verbosity") as u8;
    let collector: Collector = Collector::new()?;
    let mut slaves: Vec<&CollectorSlave> = Vec::new();
    if services == "all" {
        println!("Collecting from all supported services!");
        slaves.extend(&collector.slaves);
    } else {
        for service in services.split(",").collect::<Vec<&str>>() {
            match collector.get(&service) {
                Some(slave) => {
                    slaves.push(slave);
                },
                None => {
                    println!("No slave found for the \"{}\" service.", &service);
                }
            };
        };
    }
    if slaves.is_empty() {
        println!("No services to rip, shutting down.");
        std::process::exit(0);
    } else {
        for slave in slaves {
            println!("Starting {} collector...", &slave.domain);
            let slave_urls: Vec<String> = match slave.collect(&tags, limit) {
                Ok(slave_urls) => slave_urls,
                Err(what) => {
                    println!("Collector Slave Errored: {:#?}", what);
                    Vec::new()
                }
            };
            let mut dlr: Downloader = Downloader::new(slave_urls);
            dlr.set_path(format!("download/{}/{}", &slave.domain, tags.join("_")));
            dlr.download();
        }
        // println!("{:#?}", all_urls);
    }
    Ok(())
}
