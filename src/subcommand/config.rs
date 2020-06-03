use std::io::{self, Write};
use crate::config::{Credentials, Kindle, Config};

pub fn config() -> io::Result<()> {
    // TODO: バリデーションを追加
    let user_gmail_address = get_user_input("user_gmail_address: ")?;
    let google_application_password = get_user_input("google_application_password: ")?;
    let kindle_name = get_user_input("kindle_name: ")?;
    let kindle_mail_address = get_user_input("kindle_mail_address: ")?;

    let kindle = Kindle {
        name: kindle_name,
        mail_address: kindle_mail_address,
        default: true
    };
    let credentials = Credentials {
        user_gmail_address,
        google_application_password,
    };

    Config::save(vec![kindle], credentials)
}

fn get_user_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    Ok(input.trim().to_string())
}