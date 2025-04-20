use std::{fs, io::ErrorKind::NotFound};

use serde::{Serialize, Deserialize};
use ron::error::SpannedError;

#[derive(Serialize, Deserialize)]
pub struct VcomConfig {
    pub callsign: [u8; 6],
    // 0 (21dBm), 1 (25 dBm) or 2 (27dBm)
    pub tx_power: u8,
}

impl Default for VcomConfig {
    fn default() -> Self {
        Self { 
            callsign: *b"YH1SUP", 
            tx_power: 1
        }
    }
}

impl VcomConfig {
    pub fn load_or_default(which_vcom: u8) -> Result<Self, SpannedError> {
        match fs::read_to_string(format!("/var/vcom{}_config.ron", which_vcom)) {
            Ok(s) => ron::from_str(&s),
            Err(e) if e.kind() == NotFound => {
                let ret = Self::default();
                ret.save(which_vcom).unwrap();
                Ok(ret)
            },
            Err(e) => {
                panic!("unexpected error reading config {e:?}");
            }
        }
    }

    pub fn save(&self, which_vcom: u8) -> Result<(), std::io::Error>{
        let s = ron::to_string(self).unwrap();
        fs::write(format!("/var/vcom{}_config.ron", which_vcom), s)
    }
}