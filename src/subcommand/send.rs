use crate::mailer::{sendmail, Email, SmtpCredentials};
use crate::config::{Config};

// TODO: 複数のPDFファイルを想定した実装を追加
pub fn send(files: Vec<String>) -> Result<(), String> {
    let conf = Config::read();
    let kindle = &conf.kindles[0];

    let credentials = SmtpCredentials {
        user_gmail_address: conf.user_gmail_address,
        password: conf.google_application_password,
    };

    let mail = Email {
        to_address: kindle.mail_address.clone(),
        smtp_credentials: credentials,
        file: files[0].clone(),
    };

    sendmail(&mail)
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_send() {
        // 環境変数の値を使って実際にメールを送信して成功するかテストしている
        // TODO: ファイル添付が正常にできていることをテストしたい
        // テスト記述の参考情報
        // ローカルでSMTPサーバー起動する？
        // @see: https://github.com/lettre/lettre/tree/master/tests
        // @see: https://medium.com/@11Takanori/simple-mocking-in-rust-3a21f1fa7e0c
        let files = vec![String::from("linux.pdf")];
        let result = send(files);
        assert_eq!(result, Ok(()));
    }
}