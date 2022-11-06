#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;
use std::io::{Stdin, BufRead};
// use clap::error::ErrorFormatter;
use colored::*;
use clap::{Command, Arg};
use winconsole::console;
use std::path::Path;
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

// will repeatedly add a character until the requiered length is reached
fn rep_print(input: char, length: usize) -> String {
    let mut cnstrcd = String::new();
    for _ in 1..length {
        // println!("{}.) '{}'", i,input);
        cnstrcd.push(input);
    }
    return cnstrcd;
}

// to avoid: thread 'main' panicked at 'attempt to subtract with overflow'
fn display(input: String) -> String {
    let mut cnstrcd = String::new();

    for i in 0..25 {
        if input.chars().nth(i) != None {
            cnstrcd.push(input.chars().nth(i).unwrap());
            // cnstrcd.push(input.chars().nth(i).unwrap_or_else(&|| ' '));
        }
    }

    if cnstrcd.chars().count() == 25 && input.chars().count() != 25 {
        cnstrcd.push_str("...");
        // TOFIX
        // for i in 0..3 { 
        //     cnstrcd.push(input.chars().nth(input.chars().count() - i).unwrap())
        // }
    }
    return cnstrcd;
}

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
    
    let description = "CCCCCLLLLLIIIII - A Commandline Application";

    // clear terminal ; not recommended as of rn
    // let terminal = terminal::stdout();
    // terminal.act(Action::ClearTerminal(Clear::All)).expect("couldn't clear terminal");

    console::set_title(description).unwrap();

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
        .version("0.2.1")
        .author("by ZySvC")
        .about(description)
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
                .default_value("valid_{date}.txt")
        )
        .arg(
            Arg::new("threads")
                .short('T')
                .long("threads")
                .value_name("threads")
                .help("Number of threads to spawn")
                .default_value("100")
        )
        // .arg(
        //     Arg::new("verbose")
        //         .short('v')
        //         .long("verbose")
        //         .value_name("verbose")
        //         .help("enables verbose logging")
        // )
        .get_matches();

    // TODO: more efficient rewrite?

    // validate required arguments for their existence, if not panic
    let target_url_unformatted: String = matches.get_one::<String>("target_url").expect("failed to get target").to_string();
    let input_file_unformatted: String = matches.get_one::<String>("input_file").expect("failed to get input_file").to_string();
    let output_file_unformatted: String = matches.get_one::<String>("output_file").expect("failed to get output_file").to_string();
    let threads_unformatted: String = matches.get_one::<String>("threads").expect("failed to get threads").to_string();
    
    let target_url: ColoredString;
    let input_file: ColoredString;
    let output_file: ColoredString;
    let threads: ColoredString;
    
    // validate url with simple contains check
    if target_url_unformatted.contains("http://") || target_url_unformatted.contains("https://") {
        log::info!("{}", format!("target_url: '{}' argument is a valid url", &target_url_unformatted.to_string()));
        target_url = display(target_url_unformatted).green();
    } else {
        log::error!("{}", format!("target_url: '{}' argument is not a valid url", &target_url_unformatted.to_string()));
        target_url = display(target_url_unformatted).red();
    }

    // check if path is valid w/ std::fs
    if Path::new(&input_file_unformatted.to_string()).is_file() {
        log::info!("{}", format!("input_file: '{}' argument is a valid path", &input_file_unformatted.to_string()));
        input_file = display(input_file_unformatted).green();
    } else {
        log::error!("{}", format!("input_file: '{}' argument is not a valid path", &input_file_unformatted.to_string()));
        input_file = display(input_file_unformatted).red();
    }

    // ???????: add dialog to ask user if he wants to create a new file (extra argument?)
    // acknowledge?

    // check if path is valid w/ std::fs
    if Path::new(&output_file_unformatted.to_string()).is_file() {
        log::info!("{}", format!("output_file: '{}' argument is a valid path", &output_file_unformatted.to_string()));
        output_file = display(output_file_unformatted).green();
    } else {
        log::error!("{}", format!("output_file: '{}' argument is not a valid path", &output_file_unformatted.to_string()));
        output_file = display(output_file_unformatted).red();
    }

    // check if threads are valid by checking if the string only contains alphanumeric characters
    if threads_unformatted.chars().all(char::is_alphanumeric){
        log::info!("{}", format!("threads: '{}' argument is a alphanumeric", &threads_unformatted.to_string()));
        threads = display(threads_unformatted).green();
    } else {
        log::error!("{}", format!("threads: '{}' argument is not alphanumeric", &threads_unformatted.to_string()));
        threads = display(threads_unformatted).red();
    }


    let cfgtxt = format!("
  .-- [ Loaded Configuration ] ------------------------------------.
  |  Target         -  {}{}|
  |  Input File     -  {}{}|
  |  Output File    -  {}{}|
  |  Threads        -  {}{}|
  '----------------------------------------------------------------'",
    target_url, rep_print(' ', 45 - target_url.chars().count()),
    input_file, rep_print(' ', 45 - input_file.chars().count()),
    output_file, rep_print(' ', 45 - output_file.chars().count()),
    threads, rep_print(' ', 45 - threads.chars().count()),
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