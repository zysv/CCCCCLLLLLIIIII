#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(debug_assertions)]
    {
        simple_logger::SimpleLogger::new()
            .with_threads(true)
            .with_colors(true)
            .init()
            .unwrap();
        ::log::set_max_level(log::LevelFilter::Debug);
    }

    while true {
        log::info!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    }
}
