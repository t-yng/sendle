extern crate toml;

use std::path::Path;
use std::fs;
use dotenv::dotenv;
use std::env;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Kindle {
    pub name: String,
    pub mail_address: String,
    pub default: bool,
}

#[derive(Serialize)]
struct Credentials {
    pub user_gmail_address: String,
    pub google_application_password: String,
}

#[derive(Serialize)]
pub struct Config {
    pub kindles: Vec<Kindle>,
    pub credentials: Credentials,
}

impl Config {

    pub fn save(kindles: Vec<Kindle>, credentials: Credentials) {
        let config = Config {
            kindles,
            credentials,
        };

        let toml_str = toml::to_string(&config).unwrap();
        fs::create_dir_all(Config::dir()).unwrap();
        fs::write(Config::file(), toml_str).unwrap();
    }

    pub fn load() -> Self {
        // TODO: 開発用に.env的なファイルで設定値を持てるようにした方が良いかも
        // TODO: 例外処理
        let file = Config::file();
        let content = fs::read_to_string(file).unwrap();
        toml::from_str(content.as_str()).unwrap()
    }

    fn file() -> &'static str {
        "~/.config/sendle/config"
    }

    fn dir() -> &'static str {
        let file = Config::file();
        let p = Path::new(file);
        p.parent().unwrap().to_str().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_save() {
        // Config::file() をモック化する
        // <カレントディレクトリ>/.config/sendle/config
        // Config::save();
        // Config::read()で内容をアサーションする
    }

    #[test]
    fn test_read() {
        // 固定文字列でテスト用の設定ファイルを書き込み
        // Config::read()
        // 戻り値をアサーションする
    }
}