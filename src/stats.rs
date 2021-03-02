use std::io;
use std::sync::{Arc, Mutex};

pub fn stats_loop(silent: bool, quit: Arc<Mutex<bool>>) -> io::Result<()> {
    let mut total_bytes = 0;
    loop {
        let buffer: Vec<u8> = Vec::new();
        total_bytes += buffer.len();
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        let quit = quit.lock().unwrap();
        if *quit {
            break;
        }
    }

    eprintln!();
    Ok(())
}