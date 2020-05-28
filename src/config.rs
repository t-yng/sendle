extern crate dotenv;

use dotenv::dotenv;
use std::env;

pub struct Kindle {
    _name: String,
    pub mail_address: String,
}

pub struct Config {
    pub kindles: Vec<Kindle>,
    pub user_gmail_address: String,
    pub google_application_password: String,
}

impl Config {
    pub fn read() -> Self {
        // TODO: 環境変数でなくconfigファイルから読み込むように修正
        dotenv().ok();

        let kindle_name = env::var("KINDLE_NAME").unwrap().to_string();
        let kindle_mail_address = env::var("KINDLE_MAIL_ADDRESS").unwrap().to_string();
        let user_gmail_address = env::var("USER_GMAIL_ADDRESS").unwrap().to_string();
        let google_application_password = env::var("GOOGLE_APPLICATION_PASSWORD").unwrap().to_string();

        let kindle = Kindle {
            _name: kindle_name,
            mail_address: kindle_mail_address,
        };

        Config {
            kindles: vec![kindle],
            user_gmail_address,
            google_application_password,
        }
    }
}