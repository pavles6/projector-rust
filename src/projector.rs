use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Projector {
    config: Config,
    data: Data,
}

fn default_data() -> Data {
    return Data {
        projector: HashMap::new(),
    };
}

impl Projector {
    pub fn get_values(&self) -> HashMap<&String, &String> {
        let mut curr: Option<&std::path::Path> = Some(self.config.pwd.as_path());

        let mut paths = vec![];

        let mut values: HashMap<&String, &String> = HashMap::new();

        while let Some(p) = curr {
            paths.push(p);
            curr = p.parent();
        }

        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.projector.get(path) {
                values.extend(map);
            }
        }

        return values;
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut curr: Option<&std::path::Path> = Some(self.config.pwd.as_path());

        while let Some(p) = curr {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(val) = dir.get(key) {
                    return Some(val);
                }
            }

            curr = p.parent();
        }

        return None;
    }

    pub fn set_value(&mut self, key: String, value: String) {
        self.data
            .projector
            .entry(self.config.pwd.clone())
            .or_default()
            .insert(key, value);
    }

    pub fn remove_value(&mut self, key: &str) {
        self.data.projector.get_mut(&self.config.pwd).map(|entry| {
            entry.remove(key);
        });
    }

    pub fn from_config(config: Config) -> Self {
        if std::fs::metadata(&config.config).is_ok() {
            let contents = std::fs::read_to_string(&config.config);

            let contents = contents.unwrap_or(String::from("{\"projector:\": {}"));

            let data = serde_json::from_str(&contents);

            let data = data.unwrap_or(default_data());

            return Projector { config, data };
        }

        return Projector {
            config,
            data: default_data(),
        };
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use collection_macros::hashmap;

    use crate::config::{Config, Operation};

    use super::{Data, Projector};

    fn get_data() -> HashMap<PathBuf, HashMap<String, String>> {
        return hashmap! {
            PathBuf::from("/")=> hashmap! {
                "foo".into() => "bar1".into(),
                "fem".into()=> "is great".into(),
              },
              PathBuf::from("/foo") => hashmap! {
                "foo".into()=> "baz".into(),
                "bar".into()=> "baz".into(),
              },
              PathBuf::from("/foo/bar") => hashmap! {
                "foo".into()=> "bar3".into(),
              },
        };
    }

    fn get_projector(pwd: PathBuf) -> Projector {
        return Projector {
            data: Data {
                projector: get_data(),
            },
            config: Config {
                pwd,
                config: PathBuf::from(""),
                operation: Operation::Print(None),
            },
        };
    }

    #[test]
    fn get_value() {
        let proj = get_projector(PathBuf::from("/foo/bar"));

        assert_eq!(proj.get_value("foo"), Some(&String::from("bar3")));
        assert_eq!(proj.get_value("fem"), Some(&String::from("is great")));
    }

    #[test]
    fn get_values() {
        let proj = get_projector(PathBuf::from("/foo/bar"));

        let mut expected: HashMap<String, String> = HashMap::new();

        expected.insert("foo".to_string(), "bar3".to_string());
        expected.insert("bar".to_string(), "baz".to_string());
        expected.insert("fem".to_string(), "is great".to_string());

        for (k, v) in proj.get_values().into_iter() {
            assert_eq!(expected.get(k), Some(v));
        }
    }

    #[test]
    fn set_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));

        proj.set_value(String::from("foo"), String::from("bar4"));

        assert_eq!(proj.get_value("foo"), Some(&String::from("bar4")));
    }
    #[test]
    fn remove_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));

        proj.remove_value("foo");
        proj.remove_value("fem");

        assert_eq!(proj.get_value("foo"), Some(&String::from("baz")));
        assert_eq!(proj.get_value("fem"), Some(&String::from("is great")));
    }
}
