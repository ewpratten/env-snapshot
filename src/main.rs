use std::{collections::HashMap, fs::File, io::Read};

use clap::{crate_authors, crate_description, crate_name, App, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use std::io::Write;
use subprocess::Exec;

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("save")
                .about("Save the environment to a file")
                .arg(
                    Arg::with_name("file")
                        .help("File to export the environment data to")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("load")
                .about("Load the environment from a file & spawn a process")
                .arg(
                    Arg::with_name("file")
                        .help("File to load the environment data from")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("proc")
                        .help("Process to spawn")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("json")
                        .long("use-json")
                        .short("j")
                        .help("Use JSON instead of Flexbuffer format for parsing file")
                        .takes_value(false)
                        .required(false),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("save") {
        // Get the environment
        let mut environment = HashMap::new();
        for (key, value) in std::env::vars() {
            environment.insert(key, value);
        }

        // Write the environment to a file
        let mut s = flexbuffers::FlexbufferSerializer::new();
        environment.serialize(&mut s).unwrap();
        let mut file = File::create(matches.value_of("file").unwrap()).unwrap();
        file.write_all(s.view()).unwrap();
        println!("Environment saved");
    } else if let Some(matches) = matches.subcommand_matches("load") {
        // Read the environment from a file
        let environment: HashMap<String, String>;
        if matches.is_present("json") {
            environment = autojson::structify(matches.value_of("file").unwrap()).unwrap();
        } else {
            let mut file = File::open(matches.value_of("file").unwrap()).unwrap();
            let mut buffer = Vec::<u8>::new();
            file.read_to_end(&mut buffer).unwrap();
            let r = flexbuffers::Reader::get_root(buffer.as_slice()).unwrap();
            environment = HashMap::deserialize(r).unwrap();
        }

        // Set the environment
        println!("Setting environment...");
        for (key, value) in environment.iter() {
            std::env::set_var(key, value);
            println!("{}={}", key, value);
        }

        // Spawn the process
        println!("\n\n");
        Exec::shell(matches.value_of("proc").unwrap())
            .join()
            .unwrap();
    }
}
