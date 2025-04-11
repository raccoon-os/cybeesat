use std::{fs::File, io::{self, Read, Seek, SeekFrom, Write}, path::Path};

use rccn_usr::service::{AcceptanceResult, AcceptedTc, PusService};

use super::{command, telemetry::GetBootCounterResponse};

pub struct BootService {
    counter_file: File
}

impl BootService {
    pub fn new(counter_path: &Path) -> io::Result<Self> {
        let mut ret = Self {
            counter_file: File::options()
                .read(true)
                .write(true)
                .truncate(false)
                .create(true)
                .open(counter_path)?
        };
        ret.increment_counter()?;
        Ok(ret)
    }

    fn set(&mut self, new_value: u16) -> io::Result<()> {
        let new_content = format!("{new_value}");
        self.counter_file.seek(SeekFrom::Start(0))?;
        self.counter_file.write_all(new_content.as_bytes())?;
        self.counter_file.set_len(new_content.as_bytes().len() as u64)?;
        Ok(())
    }

    fn current_value(&mut self) -> io::Result<u16> {
        let mut content = [0u8; 10];
        self.counter_file.seek(SeekFrom::Start(0))?;
        self.counter_file.read(&mut content)?;

        Ok(atoi::atoi(&content).unwrap_or(0))
    }

    fn increment_counter(&mut self) -> io::Result<()> {
        let new_value = self.current_value()? + 1;
        self.set(new_value)
    }
}

impl PusService for BootService {
    type CommandT = command::Command;

    fn service() -> u8 {
        135
    }

    fn handle_tc(&mut self, mut tc: AcceptedTc, cmd: Self::CommandT) -> AcceptanceResult {
        match cmd {
            command::Command::GetBootCounter => tc.handle_with_tm(|| {
                match self.current_value() {
                    Ok(boot_counter) => Ok(GetBootCounterResponse { boot_counter }),
                    Err(e) => {
                        eprintln!("error getting current boot counter: {e:?}");
                        Err(())
                    }
                }
            }),
            command::Command::ResetBootCounter => tc.handle(|| {
                match self.set(1) {
                    Ok(_) => true,
                    Err(e) => {
                        eprintln!("error resetting boot counter: {e:?}");
                        false
                    }
                }
            })
        }
    }
}