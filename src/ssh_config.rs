use indexmap::IndexMap;
use std::{fs, io};

pub type HostMap = IndexMap<String, String>;

/// For now just load the hostnames and their labels.
pub fn load_ssh_config(path: &str) -> io::Result<HostMap> {
    parse_ssh_config(&fs::read_to_string(path.replace('~', env!("HOME")))?)
}

/// Parse .ssh/config to a (sorted) map.
pub fn parse_ssh_config<S: AsRef<str>>(config: S) -> io::Result<HostMap> {
    let config = config.as_ref();
    let mut map = HostMap::new();

    let mut token = String::new(); // the token we're parsing
    let mut line = vec![]; // current line
    let mut skip_line = false; // skip until EOL for comments
    let mut stanza = String::new(); // ssh config is broken into stanzas
    let mut key = true; // parsing the key or the value?

    for c in config.chars() {
        if skip_line {
            if c == '\n' {
                skip_line = false;
            }
            continue;
        }

        if c == '#' {
            // skip comments
            skip_line = true;
            if !token.is_empty() {
                line.push(token);
                token = String::new();
            }
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

            if line.is_empty() {
                continue;
            } else if line.len() != 2 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("can't parse line: {:?}", line),
                ));
            } else {
                match line[0].to_lowercase().as_ref() {
                    "host" => {
                        let parsed = &line[1];
                        // skip any Host patterns
                        if parsed.contains('*')
                            || parsed.contains('!')
                            || parsed.contains('?')
                            || parsed.contains(',')
                            || parsed.contains(' ')
                        {
                            stanza.clear();
                        } else {
                            stanza = parsed.clone();
                            // by default we assume host patterns are
                            // actual hostnames
                            map.insert(stanza.clone(), stanza.clone());
                        }
                    }
                    "hostname" => {
                        if !stanza.is_empty() {
                            map.insert(stanza.clone(), line[1].clone());
                            stanza.clear();
                        }
                    }
                    _ => {}
                }
                line.clear();
            }
        } else if (c == ' ' || c == '=') && token.is_empty() {
            // skip = and whitespace at start of value, key = value format
            continue;
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
                "midi-files.com",
            ]
        );
        assert_eq!("torrentz-r-us.com", config.get("torrentz-server").unwrap());
        assert_eq!("docker3.mycloud.net", config.get("docker3").unwrap());
        assert_eq!("192.168.1.100", config.get("nas01").unwrap());
        assert_eq!("midi-files.com", config.get("midi-files.com").unwrap());
    }
}
