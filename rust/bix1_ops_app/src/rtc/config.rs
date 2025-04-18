use std::{fs, io::ErrorKind::NotFound};

use ron::de::SpannedError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BixConfig {
    pub current_time: i64,
    pub next_reset: i64,
    pub reset_interval: u64,
    pub seconds_to_wait_before_sleeping: u32
}

impl Default for BixConfig {
    fn default() -> Self {
        Self { 
            current_time: 0,
            next_reset: 0, 
            reset_interval: 7*24*60*60, 
            seconds_to_wait_before_sleeping: 60,
        }
    }
}

impl BixConfig {
    pub fn load_or_default() -> Result<Self, SpannedError> {
        match fs::read_to_string("/var/bix1_config.ron") {
            Ok(s) => ron::from_str(&s),
            Err(e) if e.kind() == NotFound => {
                let ret = Self::default();
                ret.save().unwrap();
                Ok(ret)
            },
            Err(e) => {
                panic!("unexpected error reading config {e:?}");
            }
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error>{
        let s = ron::to_string(self).unwrap();
        fs::write("/var/bix1_config.ron", s)
    }
}

