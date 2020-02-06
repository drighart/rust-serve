use env_logger::{Builder,Env};
use log::{info,error};

use clap::{Arg, App};

#[derive(Debug)]
pub struct Arguments {
    host: String,
    port: u32,
    zipfile: String,
    pub folder: String,
}

impl Arguments {
    pub fn new() -> Arguments {
        Arguments {
            host: "localhost".to_string(),
            port: 8080,
            zipfile: "".to_string(),
            folder: "./data".to_string(),
        }
    }

    pub fn parse(&self) -> Arguments {
        let mut args = Arguments::new();

        let matches = App::new("Help-you-to-serve")
            .version("0.0.1")
            .author("David Righart <david.righart@gmail.com>")
            .about("Webserver which serves zip")
            .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Sets the port to serve on")
                .takes_value(true))
            .arg(Arg::with_name("host")
                .short("h")
                .long("host")
                .value_name("HOST")
                .help("Sets the host to serve on")
                .takes_value(true))
            .arg(Arg::with_name("zipfile")
                .short("z")
                .long("zipfile")
                .value_name("ZIPFILE")
                .help("Which zipfile to read in memory")
                .takes_value(true))
            .arg(Arg::with_name("folder")
                .short("f")
                .long("folder")
                .value_name("FOLDER")
                .help("Which folder to serve. This can not work together with the zipfile")
                .takes_value(true))

            .get_matches();

        if let Some(host) = matches.value_of("host") {
            info!("A host was passed: {}", host);
            args.host = host.to_string();
        }

        let port: () = match matches.value_of("port") {
            None => info!("Default port number is 8080."),
            Some(s) => {
                match s.parse::<u32>() {
                    Ok(n) => {
                        info!("Your favorite port must be {}.", n);
                        args.port = n;
                    },
                    Err(_) => {
                        error!("Port is not a number! {}", s);
                        return args;
                    },
                }
            }
        };

        if let Some(folder) = matches.value_of("folder") {
            info!("A folder was passed: {}", folder);
            args.folder = folder.to_string();
        } else if let Some(zipfile) = matches.value_of("zipfile") {
            info!("A zipfile was passed: {}", zipfile);
            args.zipfile = zipfile.to_string();
        }

        return args;
    }
}
