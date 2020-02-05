use env_logger::{Builder,Env};
use log::{info,error};

#[macro_use]
mod arguments;

use arguments::Arguments;


fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("I will help you to serve...");

    let args = Arguments::new().parse();

    info!("{:?}", args);
}
