mod md;
mod storage;

use md::md_monitor::MdMonitor;
use std::thread;
use std::time::Duration;
use log::{info, error};
use storage::client::Storage;
use std::path::PathBuf;


fn main() {
    env_logger::init();

    let data_path = PathBuf::from("/home/pi/raidar_data");

    let metrics_monoitor = MdMonitor::new(String::from("/sys/class/block/md0/md/"));
    let storage = Storage::new(data_path);

    loop {
        let md_stats = metrics_monoitor.get_stats();
        match md_stats {
            Ok(status) => {
                info!("md status: {:?}", status);
                let saved = &storage.save("raid", status);
                if let Err(message) = saved {
                    error!("{}", message)
                }
            },
            Err(message) => error!("{}", message)

        }
        thread::sleep(Duration::from_secs(60));
    };
}
