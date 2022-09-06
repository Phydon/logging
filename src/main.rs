use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::{error, info, warn};
use std::{error::Error, fs::File, process};
use std::thread::sleep;
use std::time::Duration;

// this logs some stuff
fn lets_log(txt: &str) {
    info!("informing: {txt}");
    warn!("warning: {txt}");
    error!("messing everything up: {txt}");
}

// this function produces an error
fn return_err() -> Result<(), Box<dyn Error>> {
    File::open("test.txt")?;

    Ok(())
}

fn main() {
    // initialize the logger
    let _logger = Logger::try_with_str("info") // log info, warn and error
        .unwrap()
        .format_for_files(detailed_format) // use timestamp for every log
        .log_to_file(FileSpec::default().suppress_timestamp()) // no timestamps in the filename
        .append() // use only one logfile
        .duplicate_to_stderr(Duplicate::Warn) // print warnings and errors also to the console
        .start()
        .unwrap();

    lets_log("Hi there");

    if let Err(err) = return_err() {
        warn!("errorrrrrr -> {err}");
        sleep(Duration::from_millis(3000));
        process::exit(1);
    }
}
