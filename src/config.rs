extern crate toml;

use std::path::Path;
use std::fs;
use serde_derive::{Serialize, Deserialize};

#[cfg(test)]
use mocktopus::macros::*;

#[derive(Serialize, Deserialize)]
pub struct Kindle {
    pub name: String,
    pub mail_address: String,
    pub default: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub user_gmail_address: String,
    pub google_application_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub kindles: Vec<Kindle>,
    pub credentials: Credentials,
}

#[cfg_attr(test, mockable)]
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

// NOTO: Mocktopusがnightlyチャンネルでしかビルドできないので
//       テスト実行はnightlyチャンネルで実行する必要がある
#[cfg(test)]
mod test {
    use super::*;
    use mocktopus::mocking::*;

    const TEST_CONFIG_DIR: &str = ".config";
    const TEST_CONFIG_FILE: &str = ".config/sendle/config";

    // TODO: テストフレームワーク導入して呼び出しは自動化したい
    fn before_each() {
        Config::file.mock_safe(|| MockResult::Return(TEST_CONFIG_FILE));
    }

    fn after_each() {
        fs::remove_dir_all(TEST_CONFIG_DIR).unwrap();
    }

    #[test]
    fn test_save() {
        before_each();
        fs::remove_dir_all(TEST_CONFIG_DIR).unwrap();

        let kindle = Kindle {
            name: "test".to_string(),
            mail_address: "test@example.com".to_string(),
            default: true,
        };
        let credentials = Credentials {
            user_gmail_address: "test@gmail.com".to_string(),
            google_application_password: "aglfurppkcfkasvs".to_string()
        };

        Config::save(vec![kindle], credentials);

        let content = fs::read_to_string(TEST_CONFIG_FILE).unwrap();
        let expected = r#"[[kindles]]
name = "test"
mail_address = "test@example.com"
default = true

[credentials]
user_gmail_address = "test@gmail.com"
google_application_password = "aglfurppkcfkasvs"
"#;

        assert_eq!(content, expected);

        after_each();
    }

    #[test]
    fn test_load() {
        before_each();

        let toml_str = r#"[[kindles]]
name = "test"
mail_address = "test@example.com"
default = true

[credentials]
user_gmail_address = "test@gmail.com"
google_application_password = "aglfurppkcfkasvs"
"#;

        fs::write(TEST_CONFIG_FILE, toml_str).unwrap();

        let config = Config::load();
        let kindle = &config.kindles[0];
        let credentials = &config.credentials;

        assert_eq!(kindle.name, "test");
        assert_eq!(kindle.mail_address, "test@example.com");
        assert_eq!(kindle.default, true);
        assert_eq!(credentials.user_gmail_address, "test@gmail.com");
        assert_eq!(credentials.google_application_password, "aglfurppkcfkasvs");

        after_each();
    }
}