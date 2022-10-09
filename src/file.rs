use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Deserialize, Serialize, Clone)]
pub struct SaveFile {
    pub nodes: Vec<Node>,
    pub name: String,
    pub latest_id: u64,
}

impl SaveFile {
    pub fn new() -> Self {
        SaveFile {
            nodes: Vec::new(),
            name: String::new(),
            latest_id: 0,
        }
    }

    pub fn save(self) {
        let json = serde_json::to_string(&self).unwrap();

        std::fs::write(appdata() + "\\saves\\" + &self.name + ".quadder", json).unwrap();
    }

    pub fn load(name: String) -> Self {
        let string_data = &std::fs::read_to_string(appdata() + "\\saves\\" + &name + ".quadder");

        let output: Self = serde_json::from_str(string_data.as_ref().unwrap()).unwrap();

        output
    }

    pub fn exists(name: String) -> bool {
        let string_data = &std::fs::read_to_string(appdata() + "\\saves\\" + &name + ".quadder");

        string_data.is_ok()
    }
}

/// Returns path of app's appdata folder
pub fn appdata() -> String {
    let appdata = dirs::data_local_dir();

    if appdata.is_none() {
        println!("Local data folder doesn't exist, cannot create files");
    }

    let mut folder = appdata.clone();
    folder.as_mut().unwrap().push("Quadder");

    if !std::path::Path::exists(&folder.clone().unwrap()) {
        std::fs::create_dir(folder.clone().unwrap()).expect("Could not create app directory");
    }

    folder.unwrap().to_str().unwrap().to_owned()
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DataStore {
    pub recents: [Option<SaveFile>; 5],
}

impl DataStore {
    pub fn new() -> Self {
        Self {
            recents: [None, None, None, None, None],
        }
    }

    pub fn add_recent(&mut self, file: SaveFile) {
        let mut data: [Option<SaveFile>; 5] = [Some(file), None, None, None, None];
        let self_data = self.clone();

        if self.recents[0].is_some() {
            data[1] = self_data.recents[0].clone();
        } else {
            self.recents = data.clone();
        }

        if self.recents[1].is_some() {
            data[2] = self_data.recents[1].clone();
        } else {
            self.recents = data.clone();
        }

        if self.recents[2].is_some() {
            data[3] = self_data.recents[2].clone();
        } else {
            self.recents = data.clone();
        }

        if self.recents[3].is_some() {
            data[4] = self_data.recents[3].clone();
        } else {
            self.recents = data;
        }
    }
}

pub fn set_data_store(data: DataStore) {
    let json = serde_json::to_string(&data).unwrap();

    std::fs::write(Path::new(&(appdata() + "\\data.json")), json)
        .expect("Could not write to data store file");
}

pub fn get_data_store() -> DataStore {
    if !Path::new(&(appdata() + "\\data.json")).exists() {
        let json = serde_json::to_string(&DataStore::new()).unwrap();
        std::fs::write(Path::new(&(appdata() + "\\data.json")), json)
            .expect("Could not create new data store file");
    }

    serde_json::from_str(
        std::fs::read_to_string(Path::new(&(appdata() + "\\data.json")))
            .expect("Data store does not exist")
            .as_str(),
    )
    .unwrap()
}
