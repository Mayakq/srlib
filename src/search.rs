use regex::Regex;
use walkdir::WalkDir;
#[derive(Debug)]
pub enum Searching {
    Absolute,
    Relative,
}
#[derive(Debug)]
pub enum Pallet {
    Regular(Regex),
    Name(String),
}
#[derive(Debug)]
pub struct Search {
    path: String,
    pallet: Pallet,
    searching: Searching,
}
#[derive(Debug)]
pub enum Error {
    LenNameZero,
    LenRegularZero,
}
impl Search {
    pub fn new(path: String, pallet: Pallet, searching: Searching) -> Self {
        Self {
            path,
            pallet,
            searching,
        }
    }
    pub fn start(&self) -> Result<Vec<String>, Error> {
        let mut vec = Vec::new();
        match &self.pallet {
            Pallet::Regular(regex) => {
                if regex.captures_len() == 0 {
                    return Err(Error::LenRegularZero);
                }
            }
            Pallet::Name(name) => {
                if name.len() == 0 {
                    return Err(Error::LenNameZero);
                }
            }
        }
        match self.searching {
            Searching::Absolute => {
                let result = self.absolute(&mut vec);
                if result.is_none() {
                    return Ok(vec);
                }
                return Err(result.unwrap());
            }
            Searching::Relative => {
                let result = self.relative(&mut vec);
                if result.is_none() {
                    return Ok(vec);
                }
                return Err(result.unwrap());
            }
        }
    }
    fn absolute(&self, vec: &mut Vec<String>) -> Option<Error> {
        match &self.pallet {
            Pallet::Regular(regex) => {}
            Pallet::Name(name) => {
                for entry in WalkDir::new(&self.path).same_file_system(true) {
                    let entry = entry.unwrap();
                    let path = entry.path().to_str().unwrap().to_string();
                    let f_name = entry.file_name().to_str().unwrap().to_string();
                    if *name == f_name {
                        vec.push(path);
                    }
                }
            }
        }
        None
    }
    fn relative(&self, vec: &mut Vec<String>) -> Option<Error> {
        match &self.pallet {
            Pallet::Regular(regex) => {}
            Pallet::Name(name) => {
                for entry in WalkDir::new(&self.path).same_file_system(true) {
                    let entry = entry.unwrap();
                    let path = entry.path().to_str().unwrap().to_string();
                    let f_name = entry.file_name().to_str().unwrap().to_string();
                    if f_name.contains(name) {
                        vec.push(path);
                    }
                }
            }
        }
        None
    }
}
