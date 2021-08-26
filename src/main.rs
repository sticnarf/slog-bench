use std::{
    env,
    fs::{self, OpenOptions},
    io::BufWriter,
    path::Path,
    thread,
    time::{Duration, Instant},
};

use slog::{slog_o, Drain, Level};
use slog_async::Async;
use slog_global::*;

fn main() {
    let log_path = env::args().skip(1).next().unwrap();
    let log_path = Path::new(&log_path);
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)
        .unwrap();
    let decorator = slog_term::PlainDecorator::new(BufWriter::new(file));
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let async_log = Async::new(drain)
        .chan_size(10240)
        .overflow_strategy(slog_async::OverflowStrategy::Block)
        .thread_name("slogger".to_string())
        .build();
    let drain = async_log.filter_level(Level::Info).fuse();
    let logger = slog::Logger::root(drain, slog_o!());
    slog_global::set_global(logger);

    let mut ths = Vec::new();
    let start = Instant::now();
    let time = Duration::from_secs(10);
    for _ in 0..4 {
        let th = thread::spawn(move || {
            while Instant::now() < start + time {
                for _ in 0..100 {
                    info!("hello!"; "where" => "right here");
                }
            }
        });
        ths.push(th);
    }
    for th in ths {
        th.join().unwrap();
    }
    let size = fs::metadata(log_path).unwrap().len();
    println!(
        "{:.2} MiB/s",
        size as f64 / (1 << 20) as f64 / start.elapsed().as_secs_f64()
    );
}
