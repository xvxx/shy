use std::{collections::BTreeMap, fs, io};

pub type HostMap = BTreeMap<String, String>;

/// For now just load the hostnames and their labels.
pub fn load_ssh_config(path: &str) -> Result<HostMap, io::Error> {
    parse_ssh_config(&fs::read_to_string(path.replace('~', env!("HOME")))?)
}

/// Parse .ssh/config to a (sorted) BTree.
pub fn parse_ssh_config<S: AsRef<str>>(config: S) -> Result<HostMap, io::Error> {
    let config = config.as_ref();
    let mut map = BTreeMap::new();

    let mut token = String::new();
    let mut line = vec![];
    let mut skip_line = false;
    let mut stanza = String::new();
    let mut key = true; // parsing the key or the value?

    for c in config.chars() {
        if skip_line {
            if c == '\n' {
                skip_line = false;
            }
            continue;
        }

        if c == '#' {
            // comment
            skip_line = true;
            line.push(token);
            token = String::new();
        } else if key && (c == ' ' || c == '=') {
            // "key = value" OR "key value" separator
            if !token.is_empty() {
                line.push(token);
                token = String::new();
                key = false;
            }
        } else if c == '\n' {
            if !token.is_empty() {
                line.push(token);
                token = String::new();
                key = true;
            }

            // newline
            if line.is_empty() {
                continue;
            } else if line.len() != 2 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("can't parse line: {:?}", line),
                ));
            } else {
                match line[0].as_ref() {
                    "Host" => stanza = line[1].clone(),
                    "Hostname" => {
                        if stanza.is_empty() {
                            return Err(io::Error::new(
                                io::ErrorKind::Other,
                                format!("can't parse line: {:?}", line),
                            ));
                        }
                        // skip catch-all
                        if stanza != "*" {
                            map.insert(stanza.clone(), line[1].clone());
                        }
                        stanza.clear();
                    }
                    _ => {}
                }
                line.clear();
            }
        } else {
            // regular char
            token.push(c);
        }
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = load_ssh_config("./tests/test_config").expect("failed to parse config");
        assert_eq!(11, config.len());

        assert_eq!(
            config.keys().cloned().collect::<Vec<_>>(),
            vec![
                "homework-server",
                "nixcraft",
                "docker1",
                "nas01",
                "docker2",
                "docker3",
                "devserver",
                "ec2-some-long-name.amazon.probably.com",
                "ec2-some-long-namer.amazon.probably.com",
                "torrentz-server",
                "midi-files.com"
            ]
        );
        assert_eq!("torrentz-r-us.com", config.get("torrentz-server").unwrap());
        assert_eq!("docker3", config.get("docker3").unwrap());
        assert_eq!("nas01", config.get("192.168.1.100").unwrap());
        assert_eq!("midi-files.com", config.get("midi-files.com").unwrap());
    }
}
