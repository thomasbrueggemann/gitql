use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

pub struct Mapfile {
    map: HashMap<String, String>
}

impl Mapfile {
    pub fn parse(filename: &str) -> Mapfile {
        
        let file = File::open(filename).unwrap();
        let lines = BufReader::new(file).lines();

        let mut local_map: HashMap<String, String> = HashMap::new();

        for line in lines {
            if let Ok(map_line) = line {
                let map_segments = map_line.split("->").collect::<Vec<&str>>();

                let key = map_segments[0].trim();
                let value = map_segments[1].trim();

                local_map.insert(key.to_string(), value.to_string());
            }
        }

        Mapfile {
            map: local_map
        }
    }

    pub fn empty() -> Mapfile {
        Mapfile {
            map: HashMap::new()
        }
    }

    pub fn map(&self, key: &str) -> String {
        let mapping = self.map.get(key);

        match mapping {
            Some(mapped) => mapped.to_string(),
            None => key.to_string()
        }
    }
}

