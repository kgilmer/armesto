use std::process;
use syslog::{Facility, Formatter3164, BasicLogger};
use log::{info, error, LevelFilter};

fn main() {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "armesto".into(),
        pid: 0,
    };

    let logger = match syslog::unix(formatter) {
        Err(e) => { println!("unable to connect to syslog: {:?}", e); return; },
        Ok(logger) => logger,
    };

    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Info)).expect("can set logger");

    info!("Starting armesto..");

    match armesto::run() {
        Ok(_) => process::exit(0),
        Err(e) => {
            error!("Unable to start armesto, aborting: {:?}", e);
            process::exit(1)
        }
    }
}
