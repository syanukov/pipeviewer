use std::io::Result;
use std::thread;

use crossbeam::channel;

use pipeviewer::{args::Args, read, stats, write};

fn main() -> Result<()> {
    let Args {
        infile,
        outfile,
        silent,
    } = Args::parse();

    let (stats_tx, stats_rx) = channel::unbounded();
    let (write_tx, write_rx) = channel::bounded(1024);

    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx, write_tx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx));

    let read_io_result = read_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();

    read_io_result?;
    write_io_result?;
    stats_io_result?;

    Ok(())
}
