use regex::Regex;
use walkdir::WalkDir;
#[derive(Debug, Default)]
pub enum Searching {
    #[default]
    Absolute,
    Relative,
}
#[derive(Debug, Clone)]
pub enum Pallet {
    Regular(Regex),
    Name(String),
}
impl Default for Pallet {
    fn default() -> Self {
        return Pallet::Name(String::with_capacity(16));
    }
}
#[derive(Debug)]
pub struct Search<'a> {
    path: &'a String,
    pallet: &'a Pallet,
    searching: &'a Searching,
}
#[derive(Debug)]
pub enum Error {
    LenNameZero,
    LenRegularZero,
    LenPathZero,
}
impl<'a> Search<'_> {
    pub fn new(path: &'a String, pallet: &'a Pallet, searching: &'a Searching) -> Search<'a> {
        Search {
            path,
            pallet,
            searching,
        }
    }
    pub fn start(&self) -> Result<Vec<String>, Error> {
        let mut vec = Vec::with_capacity(16);
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
        match &self.pallet {
            Pallet::Regular(_) => {
                let result = self.reg(&mut vec);
                if result.is_none() {
                    return Ok(vec);
                }
                return Err(result.unwrap());
            }
            Pallet::Name(_) => match self.searching {
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
            },
        }
    }
    fn reg(&self, vec: &mut Vec<String>) -> Option<Error> {
        match &self.pallet {
            Pallet::Regular(re) => {
                for entry in WalkDir::new(&self.path).same_file_system(true) {
                    let entry = entry.unwrap();
                    let f_path = entry.path().to_str();
                    let f_name = entry.file_name().to_str();
                    if let None = f_path {
                        return Some(Error::LenPathZero);
                    }
                    if let None = f_name {
                        return Some(Error::LenNameZero);
                    }
                    Search::reg_search(
                        vec,
                        f_name.unwrap().to_string(),
                        re,
                        f_path.unwrap().to_string(),
                    );
                }
            }
            Pallet::Name(_name) => {
                unreachable!();
            }
        }
        None
    }
    fn reg_search(vec: &mut Vec<String>, f_name: String, re: &Regex, path: String) {
        for _ in re.find_iter(&f_name) {
            vec.push(path.clone());
        }
    }
    fn absolute(&self, vec: &mut Vec<String>) -> Option<Error> {
        match &self.pallet {
            Pallet::Regular(_) => {}
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
            Pallet::Regular(_) => {}
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
