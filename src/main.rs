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
struct AppConfig {
    pub target_url: Box<str>,
    pub input_file: Box<str>,
    pub output_file: Box<str>,
    pub threads: usize,
    pub proxies_file: Box<str>,
    pub log_file: Box<str>,
    pub delay: u64,
    pub timeout: u64,
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
            Arg::new("target_url")
                .short('t')
                .long("target")
                .value_name("target_url")
                .help("Target ip or url for the timeout tests")
                .default_value("https://ifconfig.me")
                .num_args(1)
        )
        .arg(
            Arg::new("input_file")
                .short('i')
                .long("input")
                .value_name("input_file")
                .help("File containing your input list")
                .action(ArgAction::Set)
                .num_args(1)
        )
        .arg(
            Arg::new("output_file")
                .short('o')
                .long("output")
                .value_name("output_file")
                .help("File to write all valid parts of the list to")
                .default_value("valid_proxies.txt")
                .action(ArgAction::Set)
                .num_args(1)
        )
        .arg(
            Arg::new("threads")
                .short('T')
                .long("threads")
                .value_name("threads")
                .help("Number of threads to spawn")
                .action(ArgAction::Set)
                .num_args(1)
        )
        .arg(
            Arg::new("proxy_file")
                .short('p')
                .long("proxies")
                .value_name("proxy_file")
                .help("File containing a line-separated list of proxies")
                .action(ArgAction::Set)
                .num_args(1),
        )
        // .arg(
        //     Arg::new("verbose")
        //         .short('v')
        //         .long("verbose")
        //         .value_name("verbose")
        //         .help("enables verbose logging")
        // )
        .arg(
            Arg::new("log")
                .short('l')
                .long("log")
                .value_name("log_file")
                .help("file to output logs to")
                .action(ArgAction::Set)
                .num_args(1)
        )
        .get_matches();
    // let cfg = Command::new(AppConfig {
    //     target_url: matches.value_of("target").unwrap().into(),
    //     input: matches.value_of("input").unwrap().into(),
    //     output: matches.value_of("output").unwrap().into(),
    //     delay: 1000
    //         / matches
    //             .value_of("cons_per_sec")
    //             .map(|x| x.parse().expect("Invalid amount of connections"))
    //             .unwrap_or(30),
    //     cores: matches
    //         .value_of("cores")
    //         .map(|x| x.parse().expect("Invalid amount of cores"))
    //         .unwrap_or_else(num_cpus::get),
    //     timeout: matches
    //         .value_of("timeout")
    //         .map(|x| x.parse().expect("Invalid timeout"))
    //         .unwrap_or(5),
    // })
    println!("{}", format!("    [{}] Press [{}] to continue", "+".bright_blue(), "ENTER".white().bold()).white());

    // wait for newline input
    io::stdin().discard_until_newline().unwrap();

    // while true {
    //     log::info!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    // }
}
