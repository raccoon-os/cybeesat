use std::{
    path::Path, str, sync::{mpsc::Receiver, Arc, Mutex}, thread::{self, sleep, JoinHandle}, time::Duration
};

use regex::Regex;

use super::telemetry::APRSTelemetry;

pub fn spawn(
    path: &Path,
    telemetry: Arc<Mutex<APRSTelemetry>>,
    cmd_rx: Receiver<String>,
) -> std::io::Result<JoinHandle<()>> {
    let mut port = serial2::SerialPort::open(path, 9600)?;
    port.set_read_timeout(Duration::from_millis(1000))?;

    Ok(thread::spawn(move || {
        let mut buf = [0u8; 1000];
        let mut i = 0;
        // Request telemetry once, at start-up.
        let _ = port.write(b"\r\n");
        loop {
            if let Ok(bytes_read) = port.read(&mut buf[i..]) {
                if bytes_read > 0 {
                    let last_idx = i + bytes_read - 1;
                    println!("last byte {}", buf[last_idx]);
                    if buf[last_idx] != b'\n' {
                        // Partial read, store in buffer and parse the buffer
                        // at the next read
                        i += bytes_read;
                        continue;
                    }
                    let mut tm = telemetry.lock().unwrap();
                    parse_buf(&buf[..last_idx], &mut tm);
                    
                    // Buffer consumed, reset last index
                    i = 0;

                    println!("Telemetry is {:?}", tm);
                }
            }

            if let Ok(cmd) = cmd_rx.try_recv() {
                if let Err(e) = port.write(cmd.as_bytes()) {
                    println!("Error sending command '{cmd}': {e:?}");
                }
                sleep(Duration::from_millis(100));
            } /*else {
                // Send \n to request telemetry
                let _ = port.write(b"\n");
            }*/
        }
    }))
}

fn parse_buf(buf: &[u8], telemetry: &mut APRSTelemetry) -> bool {
    let mut ok = true;

    for line in str::from_utf8(&buf).unwrap_or_default().lines() {
        ok &= parse(line, telemetry);
    }

    ok
}

fn parse(line: &str, telemetry: &mut APRSTelemetry) -> bool {
    let tlm_regex = Regex::new(r"AT\+([^\=]+)=([^\r\n]*)").unwrap();

    if let Some(captures) = tlm_regex.captures(line) {
        if captures.len() != 3 {
            println!("Unknown line '{line}'");
            return false;
        }

        let param = &captures[1];
        let value = &captures[2];

        match param {
            "CALL" => {
                telemetry.CALLSIGN = value.to_string();
            }
            "BTIME" => {
                telemetry.BEACON_TIME = u16::from_str_radix(value, 10).unwrap_or(u16::MAX);
            }
            "BEACON" => {
                telemetry.BEACON_MESSAGE = value.to_string();
            }
            "PWR" => {
                telemetry.HIGH_POWER = value == "H";
            }
            "DIGI1" => {
                telemetry.DIGI1_ENABLED = value == "ON";
            }
            "DIGI2" => {
                telemetry.DIGI2_ENABLED = value == "ON";
            }
            _ => {
                //println!("Unknown parameter '{param}'");
                return false;
            }
        }
    } else {
        println!("unknown line '{line}'");
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_parse_aprs_file() {
        // Initialize telemetry with default values
        let mut telemetry = APRSTelemetry::default();

        // Open the sample file
        let mut file = File::open("dev/aprs.txt").expect("Failed to open aprs.txt file");

        let mut buf = [0u8; 5000];
        file.read(&mut buf).unwrap();
        parse_buf(&buf, &mut telemetry);

        // Verify the expected values were parsed correctly
        assert_eq!(telemetry.CALLSIGN, "DE9TST");
        assert_eq!(telemetry.BEACON_TIME, 120);
        assert_eq!(telemetry.HIGH_POWER, false); // "L" for low power
        assert_eq!(telemetry.DIGI1_ENABLED, true); // "ON"
        assert!(telemetry.BEACON_MESSAGE.contains("Test beacon message"));
    }
}
