#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;
use std::io::{Stdin, BufRead};
use colored::*;
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

    println!("{}", format!("    [{}] Press [{}] to continue", "+".bright_blue(), "ENTER".white().bold()).white());

    // wait for newline input
    io::stdin().discard_until_newline().unwrap();

    while true {
        log::info!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    }
}
