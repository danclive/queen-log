use queen_log::*;
use log::*;

fn main() {
    init(LevelFilter::max()).unwrap();

    trace!(target: "lala", "{:?}", "hello");
    debug!("{:?}", "hello");
    info!("{:?}", "hello");
    warn!("{:?}", "hello");
    error!("{:?}", "hello");
}
