use env_logger::{Builder,Env};
use log::{info,error};

use clap::{Arg, App};

#[derive(Debug)]
pub struct Arguments {
    host: String,
    port: u32,
}

impl Arguments {
    pub fn new() -> Arguments {
        Arguments {
            host: "localhost".to_string(),
            port: 8080,
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
            .get_matches();

        // Gets a value for config if supplied by user, or defaults to "default.conf"
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

        return args;
    }
}
