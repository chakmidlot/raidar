use std::fs;
use std::str::FromStr;
use serde::Serialize;


#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct MdStats {
    pub array_state: String,
    pub component_size: u64,
    pub degraded: u32,
    pub mismatch_cnt: u64,
    pub raid_disks: u32,
    pub sync_action: String,
    pub sync_completed: Option<Progress>,
    pub sync_speed: String,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Progress {
    pub done: u64,
    pub total: u64
}


pub struct MdMonitor {
    base_path: String
}

impl MdMonitor {
    pub fn new(base_path: String) -> MdMonitor {
        MdMonitor {base_path}
    }

    pub fn get_stats(&self) -> Result<MdStats, String> {
        Ok(MdStats {
            array_state: self.get_value(String::from("array_state"))?,
            component_size: self.get_value(String::from("component_size"))?,
            sync_action: self.get_value(String::from("sync_action"))?,
            sync_completed: self.get_completed()?,
            mismatch_cnt: self.get_value(String::from("mismatch_cnt"))?,
            degraded: self.get_value(String::from("degraded"))?,
            raid_disks: self.get_value(String::from("raid_disks"))?,
            sync_speed: self.get_value(String::from("sync_speed"))?
        })
    }

    fn get_value<T: FromStr>(&self, path: String) -> Result<T, String> {
        fs::read_to_string(format!("{}/{}", self.base_path, path))
            .map_err(|_| format!("Failed to read {}", path))?
            .trim()
            .parse().map_err(|_| format!("Failed to parse {}", path))
    }

    fn get_completed(&self) -> Result<Option<Progress>, String> {
        let progress = self.get_value::<String>(String::from("sync_completed"))?;
        let progress = progress.split(" / ").collect::<Vec<_>>();

        match progress.as_slice() {
            [done, total] => {
                let done = done.parse().map_err(|_| "Unexpected format of \"sync_completed\"")?;
                let total = total.parse().map_err(|_| "Unexpected format of \"sync_completed\"")?;
                Ok(Some(Progress{done, total}))
            },
            _ => Err(String::from("Unexpected format of \"sync_completed\""))
        }
    }
}


#[test]
fn get_metrics() {
    let md = MdMonitor::new(String::from("src/md/sample_data/")).get_stats();

    let expected = Ok(MdStats {
        array_state: String::from("clean"), component_size: 3906851776, degraded: 0,
        mismatch_cnt: 100, raid_disks: 2, sync_action: String::from("check"),
        sync_completed: Some(Progress { done: 102982528, total: 7813703552 }), sync_speed: String::from("sync_speed")
    });

    assert_eq!(md, expected);
}
