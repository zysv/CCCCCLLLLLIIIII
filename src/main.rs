#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;
use std::io::{Stdin, BufRead};
use colored::*;
use clap::{Command, Arg};
// use terminal::{Action, Clear};

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

fn rep_print(input: char, length: usize) -> String {
    let mut cnstrcd = String::new();
    for _ in 1..length {
        // println!("{}.) '{}'", i,input);
        cnstrcd.push(input);
    }
    return cnstrcd;
}
// struct AppConfig {
//     pub target_url: Box<str>,
//     pub input_file: Box<str>,
//     pub output_file: Box<str>,
//     pub threads: usize,
//     pub proxies_file: Box<str>,
//     pub log_file: Box<str>,
//     pub delay: u64,
//     pub timeout: u64,
// }

#[tokio::main]
async fn main() {
    // debug logger
    #[cfg(debug_assertions)]
    {
        simple_logger::SimpleLogger::new()
            .with_threads(true)
            .with_colors(true)
            .init()
            .unwrap();
        ::log::set_max_level(log::LevelFilter::Debug);
    }
    
    // clear terminal ; not recommended as of rn
    // let terminal = terminal::stdout();
    // terminal.act(Action::ClearTerminal(Clear::All)).expect("couldn't clear terminal");

    // welcome screen
    let cli_txt = format!("
  _________ _________ _________  ___     ___     ___     ___ ___ ___
  \\_   ___ \\\\_   ___ \\\\_   ___ \\|   |   |   |   |   |   |   |   |   |  -  {}{}{}
  /    \\  \\//    \\  \\//    \\  \\/|   |   |   |   |   |   |   |   |   |
  \\     \\____     \\____     \\____   |___|   |___|   |___|   |   |   |
   \\______  /\\______  /\\______  /______ \\______ \\______ \\___|___|___|
            \\/        \\/        \\/       \\/      \\/      \\/            ", 
    "github".bright_black(), "/".white(), "zysv".bright_blue());
    
    // print welcome screen
    println!("{}", cli_txt.bright_red());

    let matches = Command::new("cccccllllliiiii")
        .version("0.1.9")
        .author("by ZySvC")
        .about("helllllllllooooooooooooooo")
        .arg(
            Arg::new("target_url")
                .short('t')
                .long("target")
                .value_name("target_url")
                .help("Target ip or url for the timeout tests")
                .default_value("https://ifconfig.me")
        )
        .arg(
            Arg::new("input_file")
                .short('i')
                .long("input")
                .value_name("input_file")
                .help("File containing your input list")
        )
        .arg(
            Arg::new("output_file")
                .short('o')
                .long("output")
                .value_name("output_file")
                .help("File to write all valid parts of the list to")
                .default_value("valid_proxies.txt")
        )
        .arg(
            Arg::new("threads")
                .short('T')
                .long("threads")
                .value_name("threads")
                .help("Number of threads to spawn")
        )
        // .arg(
        //     Arg::new("verbose")
        //         .short('v')
        //         .long("verbose")
        //         .value_name("verbose")
        //         .help("enables verbose logging")
        // )
        .get_matches();

    // TODO: Argument validation (colorful?)
    
    // validate required arguments for their existence, if not panic
    let target_url = matches.get_one::<String>("target_url").expect("failed to get target");
    let input_file = matches.get_one::<String>("input_file").expect("failed to get input_file");
    let output_file = matches.get_one::<String>("output_file").expect("failed to get output_file");
    let threads = matches.get_one::<String>("threads").expect("failed to get threads");

    let cfgtxt = format!("
  .-- [ Loaded Configuration ] ------------------------------------.
  |  Target         -  {}{}|
  |  Input File     -  {}{}|
  |  Output File    -  {}{}|
  |  Threads        -  {}{}|
  '----------------------------------------------------------------'",
    target_url.green(), rep_print(' ', 45 - target_url.chars().count()),
    input_file.green(), rep_print(' ', 45 - input_file.chars().count()),
    output_file.green(), rep_print(' ', 45 - output_file.chars().count()),
    threads.green(), rep_print(' ', 45 - threads.chars().count()),
    );
    
    // print configuration
    println!("{}\n", cfgtxt);

    println!("{}", format!("  [{}] Press [{}] to continue", "+".bright_blue(), "ENTER".white().bold()).white());

    // wait for newline input
    io::stdin().discard_until_newline().unwrap();

    // while true {
    //     log::info!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    // }
}
