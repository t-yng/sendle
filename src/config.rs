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

    pub fn read() -> Self {
        // TODO: 環境変数でなくconfigファイルから読み込むように修正
        dotenv().ok();

        let kindle_name = env::var("KINDLE_NAME").unwrap().to_string();
        let kindle_mail_address = env::var("KINDLE_MAIL_ADDRESS").unwrap().to_string();
        let user_gmail_address = env::var("USER_GMAIL_ADDRESS").unwrap().to_string();
        let google_application_password = env::var("GOOGLE_APPLICATION_PASSWORD").unwrap().to_string();

        let kindle = Kindle {
            name: kindle_name,
            mail_address: kindle_mail_address,
            default: false,
        };

        let credentials = Credentials {
            user_gmail_address,
            google_application_password,
        };

        Config {
            kindles: vec![kindle],
            credentials,
        }
    }

    fn file<'a>() -> &'a str {
        "~/.config/sendle/config"
    }

    fn dir<'a>() -> &'a str {
        let p = Path::new(Config::file());
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