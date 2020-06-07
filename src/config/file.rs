extern crate toml;
extern crate home;

use std::path::Path;
use std::fs;
use std::io;

#[cfg(test)]
use mockall::{automock, predicate::*};

pub struct ConfigFile {}

// mockallとcfg_ifの影響でrustcにdead_codeと判定されていますが、複数箇所で参照されています。
#[allow(dead_code)]
#[cfg_attr(test, automock)]
impl ConfigFile {
    pub fn new() -> Self {
        ConfigFile {}
    }

    pub fn write(&self, content: &str) -> io::Result<()> {
        fs::create_dir_all(self.dir_path_str()).unwrap();
        fs::write(self.path_str(), content)
    }

    pub fn read(&self) -> io::Result<String> {
        let file = self.path_str();
        fs::read_to_string(file)
    }

    pub fn path_str(&self) -> String {
        let home_path = home::home_dir().unwrap();
        let home = home_path.to_str().unwrap();
        format!("{}/.config/sendle/config", home)
    }

    pub fn dir_path_str(&self) -> String {
        let file = self.path_str();
        let p = Path::new(&file);
        p.parent().unwrap().to_str().unwrap().to_string()
    }
}
