mod file;

extern crate toml;
extern crate home;
extern crate cfg_if;

use std::io;
use serde_derive::{Serialize, Deserialize};

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use self::file::MockConfigFile as ConfigFile;
    } else {
        pub use self::file::ConfigFile;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kindle {
    pub name: String,
    pub mail_address: String,
    pub default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    pub user_gmail_address: String,
    pub google_application_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub credential: Credential,
    pub kindles: Vec<Kindle>,
}

impl Config {

    pub fn new(credential: Credential, kindles: Vec<Kindle>) -> Self {
        Config {
            credential,
            kindles,
        }
    }

    pub fn save(&self, file: &ConfigFile) -> io::Result<()>{
        let toml_str = toml::to_string(&self).unwrap();
        file.write(&toml_str)
    }

    pub fn load(file: &ConfigFile) -> Self {
        // TODO: 開発用に.env的なファイルで設定値を持てるようにした方が良いかも
        // TODO: 例外処理
        let content = file.read().unwrap();
        toml::from_str(content.as_str()).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_save() {
        let credential = Credential {
            user_gmail_address: "test@gmail.com".to_string(),
            google_application_password: "aglfurppkcfkasvs".to_string()
        };
        let kindle = Kindle {
            name: "test".to_string(),
            mail_address: "test@example.com".to_string(),
            default: true,
        };
        let config = Config::new(credential, vec![kindle]);

        let expected = r#"[credential]
user_gmail_address = "test@gmail.com"
google_application_password = "aglfurppkcfkasvs"

[[kindles]]
name = "test"
mail_address = "test@example.com"
default = true
"#;

        let mut mock = ConfigFile::default();
        mock.expect_write()
            .with(eq(expected))
            .times(1)
            .returning(|_| Ok(()) );

        assert_eq!((), config.save(&mock).unwrap());
    }

    #[test]
    fn test_load() {
        let mut mock = ConfigFile::default();
        mock.expect_read()
            .returning(|| {
                let toml_str = r#"[credential]
                user_gmail_address = "test@gmail.com"
                google_application_password = "aglfurppkcfkasvs"

                [[kindles]]
                name = "test"
                mail_address = "test@example.com"
                default = true
                "#;

                Ok(toml_str.to_string())
            });

        let config = Config::load(&mock);
        let kindle = &config.kindles[0];
        let credentials = &config.credential;

        assert_eq!(kindle.name, "test");
        assert_eq!(kindle.mail_address, "test@example.com");
        assert_eq!(kindle.default, true);
        assert_eq!(credentials.user_gmail_address, "test@gmail.com");
        assert_eq!(credentials.google_application_password, "aglfurppkcfkasvs");
    }
}