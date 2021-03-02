use std::io::Result;
use std::sync::{Arc, Mutex};
use std::thread;

use pipeviewer::{args::Args, read, stats, write};

fn main() -> Result<()> {
    let Args { infile, outfile, silent } = Args::parse();

    let quit = Arc::new(Mutex::new(false));
    let (quit1, quit2, quit3) = (Arc::clone(&quit), Arc::clone(&quit), Arc::clone(&quit));

    let read_handle = thread::spawn(move || read::read_loop(&infile, quit1));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, quit2));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, quit3));

    let read_io_result = read_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();

    read_io_result?;
    write_io_result?;
    stats_io_result?;

    Ok(())
}
