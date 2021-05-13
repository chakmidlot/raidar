use std::fs::OpenOptions;
use std::io::Write;
use serde::Serialize;
use serde_json::json;
use chrono::Utc;
use std::path::PathBuf;


pub struct Storage {
    path: PathBuf
}

impl Storage {

    pub fn new(path: PathBuf) -> Storage {
        Storage {path}
    }

    pub fn save<T: Serialize>(&self, metric_name: &str, data: T) -> Result<(), String> {
        let file_path = self.path.join(format!("{}.json", metric_name));

        let mut file = OpenOptions::new().create(true).append(true).open(file_path)
            .map_err(|x| x.to_string())?;

        let value = json!({
            "timestamp": Utc::now().to_rfc3339(),
            "payload": data
        }).to_string();

        file.write(value.as_bytes())
            .map_err(|x| x.to_string())?;

        file.write(b"\n")
            .map_err(|x| x.to_string())?;

        Ok(())
    }
}



#[test]
fn test_save() {
    use crate::md::md_monitor::{MdStats, Progress};
    use std::fs;
    use tempdir::TempDir;
    use serde_json::Value;

    let data1 = MdStats {
        array_state: String::from("clean"), component_size: 3906851776, degraded: 0,
        mismatch_cnt: 100, raid_disks: 2, sync_action: String::from("check"),
        sync_completed: Some(Progress { done: 102982528, total: 7813703552 }), sync_speed: String::from("sync_speed")
    };
    let data2 = MdStats {
        array_state: String::from("clean"), component_size: 3906851776, degraded: 0,
        mismatch_cnt: 100, raid_disks: 3, sync_action: String::from("check"),
        sync_completed: Some(Progress { done: 102982528, total: 7813703552 }), sync_speed: String::from("sync_speed")
    };

    let dir = TempDir::new("raidar").unwrap();
    let dir_path = dir.path();

    let storage = Storage::new(dir_path.clone().to_owned());
    storage.save("raid", data1).unwrap();
    storage.save("raid", data2).unwrap();

    let result: Vec<Value> = fs::read_to_string(dir_path.join("raid.json")).unwrap()
        .trim().split("\n")
        .map(|x| serde_json::from_str(x).unwrap())
        .collect();

    let expected = vec![
        json!({
            "array_state":"clean", "component_size":3906851776_i64, "degraded":0, "mismatch_cnt":100,
            "raid_disks":2, "sync_action":"check", "sync_completed":
                {"done":102982528_i64, "total":7813703552_i64
            },
            "sync_speed":"sync_speed"}),

        json!({
            "array_state":"clean", "component_size":3906851776_i64, "degraded":0, "mismatch_cnt":100,
            "raid_disks":3, "sync_action":"check", "sync_completed":
                {"done":102982528_i64, "total":7813703552_i64
            },
            "sync_speed":"sync_speed"})
    ];

    for (result, expected) in result.iter().zip(&expected) {
        assert_eq!(result.get("payload").unwrap(), expected);
    }

}
