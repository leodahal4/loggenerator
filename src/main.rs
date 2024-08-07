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
use std::{env, thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let save_in_log = env::var("SAVE_IN_FILE").is_ok();
    let stdout = ConsoleAppender::builder().build();
    let mut config_builder = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)));
    let mut root_builder = Root::builder().appender("stdout");
    if save_in_log {
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
            .build("output.log")?;
        config_builder = config_builder
            .appender(Appender::builder().build("logfile", Box::new(logfile)));
        root_builder = root_builder.appender("logfile");
    }
    let config = config_builder
        .build(root_builder.build(LevelFilter::Info))?;
    log4rs::init_config(config)?;
    let mut counter = 1;
    loop {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S:%f").to_string();
        let log_entry = format!("Log entry #{}: Generated at {}", counter, timestamp);
        if save_in_log {
            info!("{}", log_entry);
        } else {
            println!("{}", log_entry);
        }
        counter += 1;
        thread::sleep(Duration::from_nanos(1));
        if counter == 10000{
            println!("10000 lines printed, so now sleeping for 5 seconds");
            thread::sleep(Duration::from_secs(5));
            counter = 1;
        }
    }
}