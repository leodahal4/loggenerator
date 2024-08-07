extern crate chrono;
extern crate log;
extern crate log4rs;
use chrono::Local;
use log::{info, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::{
    sync::mpsc,
    thread,
    time::Duration,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = ConsoleAppender::builder().build();
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("output.log")?;
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
            .appender("stdout")
            .appender("logfile")
            .build(LevelFilter::Info))?;
    log4rs::init_config(config)?;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        while let Ok(log_entry) = rx.recv() {
            info!("{}", log_entry);
        }
    });
    let mut counter = 1;
    loop {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("Log entry #{}: Generated at {}", counter, timestamp);
        tx.send(log_entry).unwrap();
        counter += 1;
        thread::sleep(Duration::from_secs(5));
    }
}