use std::{fs::OpenOptions, io::BufWriter, thread};

use slog::{slog_o, Drain};
use slog_async::Async;
use slog_global::info;

fn main() {
    let log_path = "/tmp/mylog.log";
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
    let drain = async_log.fuse();
    let logger = slog::Logger::root(drain, slog_o!());
    slog_global::set_global(logger);

    let mut ths = Vec::new();
    for _ in 0..4 {
        let th = thread::spawn(move || loop {
            info!("hello!"; "where" => "right here");
        });
        ths.push(th);
    }
    for th in ths {
        th.join().unwrap();
    }
}
