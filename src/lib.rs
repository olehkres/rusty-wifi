use std::process::Command;
use std::str;

#[derive(Debug, PartialEq, Eq)]
pub struct WiFi {
    ssid: String,
    bssid: String,
    bandwidth: i32,
    channel: i32,
    signal: i32,
}

impl Default for WiFi {
    fn default() -> Self {
        WiFi::new(&"Default", &"FF:FF:FF:FF:FF:FF", 20, 0, 0)
    }
}

impl WiFi {
    pub fn new(
        ssid: &impl ToString,
        bssid: &impl ToString,
        bandwidth: i32,
        channel: i32,
        signal: i32,
    ) -> Self {
        WiFi {
            ssid: ssid.to_string(),
            bssid: bssid.to_string(),
            bandwidth,
            channel,
            signal,
        }
    }

    pub fn ssid(&self) -> &String {
        &self.ssid
    }

    pub fn bssid(&self) -> &String {
        &self.bssid
    }

    pub fn bandwidth(&self) -> &i32 {
        &self.bandwidth
    }

    pub fn channel(&self) -> &i32 {
        &self.channel
    }

    pub fn signal(&self) -> &i32 {
        &self.signal
    }

    pub fn scan() -> Vec<Self> {
        let nmcli_raw = Command::new("nmcli")
            .arg("-t")
            .arg("-f")
            .arg("SSID,CHAN,SIGNAL,BSSID")
            .arg("-m")
            .arg("multiline")
            .arg("device")
            .arg("wifi")
            .arg("list")
            .arg("--rescan")
            .arg("yes")
            .output()
            .unwrap();
        let nmcli_raw = str::from_utf8(&nmcli_raw.stdout).unwrap();

        return Self::parse_nmcli(&nmcli_raw, 4);
    }

    fn parse_nmcli(input: &str, params_count: i32) -> Vec<WiFi> {
        let input = input.replace("\\:", ":");

        let mut current = WiFi {
            ..Default::default()
        };
        let mut wifis = vec![];

        let mut i = 0;
        let lines = input.lines();
        // for every param
        for l in lines {
            if i > params_count - 1 {
                // push current wifi to vec and create new to complete.
                wifis.push(current);
                current = WiFi {
                    ..Default::default()
                };
                i = 0;
            }
            i += 1;

            // define key:value
            if let Some(p) = l.chars().position(|c| c == ':') {
                let k = &l[..p];
                let v = &l[p + 1..];

                match k {
                    "SSID" => current.ssid = v.parse().unwrap_or("ERROR".to_string()),
                    "CHAN" => current.channel = v.parse().unwrap_or(0),
                    "SIGNAL" => current.signal = v.parse().unwrap_or(0),
                    // TODO: When 1.46 out check if it is indeed BW
                    "BW" => current.bandwidth = v.parse().unwrap_or(0),
                    "BSSID" => current.bssid = v.parse().unwrap_or("ERROR".to_string()),
                    x => eprintln!("Unknown parameter: {x}"),
                }
            } else {
                eprintln!("Cant find param in line: {l}")
            };
        }
        return wifis;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn wifi_parser_test() {
        // let raw = "SSID:Test\nCHAN:4\nSIGNAL:90\nBSSID:82\\:45\\:58\\:31\\:B1\\:64";
        // let wifi = &parse_nmcli(raw)[0];
        // assert_eq!(
        //     wifi,
        //     &WiFi {
        //         ssid: "Test".to_string(),
        //         bssid: "82:45:58:31:B1:64".to_string(),
        //         channel: 4,
        //         signal: 90,
        //         bandwidth: 20
        //     }
        // );
    }
}
