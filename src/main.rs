#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;
use std::io::{Stdin, BufRead};
use colored::*;
use clap::{Command, Arg, ArgAction};

// https://stackoverflow.com/questions/39351453/is-there-a-standard-way-of-discarding-input-in-rust
pub trait DiscardUntil {
    fn discard_until_newline(&mut self) -> Result<(), io::Error>;
}

impl DiscardUntil for Stdin {
    fn discard_until_newline(&mut self) -> Result<(), io::Error> {
        let mut buffered = self.lock();
        let amount = {
            let data = buffered.fill_buf()?;
            data.len()
        };
        buffered.consume(amount);
        Ok(())
    }
}


#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        simple_logger::SimpleLogger::new()
            .with_threads(true)
            .with_colors(true)
            .init()
            .unwrap();
        ::log::set_max_level(log::LevelFilter::Debug);
    }
    let cli_txt = format!("
    _________ _________ _________  ___     ___     ___     ___ ___ ___
    \\_   ___ \\\\_   ___ \\\\_   ___ \\|   |   |   |   |   |   |   |   |   |  -  {}{}{}
    /    \\  \\//    \\  \\//    \\  \\/|   |   |   |   |   |   |   |   |   |
    \\     \\____     \\____     \\____   |___|   |___|   |___|   |   |   |
     \\______  /\\______  /\\______  /______ \\______ \\______ \\___|___|___|
            \\/        \\/        \\/       \\/      \\/      \\/            
    ", "github".bright_black(), "/".white(), "zysv".bright_blue());
    
    println!("{}", cli_txt.bright_red());

    let matches = Command::new("cccccllllliiiii")
        .version("0.1.7")
        .author("by ZySvC")
        .about("helllllllllooooooooooooooo")
        .arg(
            Arg::new("input_file")
                .short('i')
                .long("input")
                .help("file containing your input list")
                .action(ArgAction::Set)
                .num_args(1)
        )
        .arg(
            Arg::new("output_file")
                .short('o')
                .long("output")
                .help("file to write all valid parts of the list to")
                .action(ArgAction::Set)
                .num_args(1)
        )
        .arg(
            Arg::new("proxies")
                .short('p')
                .long("proxies")
                .value_name("proxyfile")
                .help("file containing a line-separated list of proxies")
                .action(ArgAction::Set)
                .num_args(1),
        )
        .arg(
            Arg::new("jobs")
                .short('j')
                .long("jobs")
                .value_name("# jobs")
                .help("number of threads to spawn (defaults to # cpus available)")
                .action(ArgAction::Set)
                .num_args(1)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("enables verbose logging")
        )
        .arg(
            Arg::new("log")
                .short('l')
                .long("log")
                .value_name("log file")
                .help("file to output logs to")
                .action(ArgAction::Set)
                .num_args(1)
        )
        .get_matches();

    println!("{}", format!("    [{}] Press [{}] to continue", "+".bright_blue(), "ENTER".white().bold()).white());

    // wait for newline input
    io::stdin().discard_until_newline().unwrap();

    while true {
        log::info!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    }
}
