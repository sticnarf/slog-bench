use std::{fs::OpenOptions, thread};

use slog::*;
use slog_async::Async;

fn main() {
    let log_path = "/tmp/mylog.log";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)
        .unwrap();
    let decorator = slog_term::PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let (async_log, _guard) = Async::new(drain)
        .chan_size(10240)
        .overflow_strategy(slog_async::OverflowStrategy::Block)
        .thread_name("slogger".to_string())
        .build_with_guard();
    let drain = async_log.fuse();
    let log = slog::Logger::root(drain, o!());

    let mut ths = Vec::new();
    for _ in 0..4 {
        let log = log.clone();
        let th = thread::spawn(move || loop {
            info!(log, "hello!"; "where" => "right here");
        });
        ths.push(th);
    }
    for th in ths {
        th.join().unwrap();
    }
}
