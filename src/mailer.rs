use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::{authentication::Credentials};

// TODO: 送信先のKindel端末のメールアドレスを指定する
// TODO: ユーザーのメールアドレス（設定ファイルから取得）
// TODO: ユーザーのパスワード（設定ファイルから取得）
pub fn sendmail(to_address: &str, user_gmail_address: &str, password: &str) -> Result<(), String> {
    let smtp_server = "smtp.googlemail.com";

    let email = Message::builder()
        .to(to_address.parse().unwrap())
        .from(user_gmail_address.parse().unwrap())
        .subject("send some files to my kindle by kindle-push")
        .body("")
        .unwrap();

    let credentials = Credentials::new(
        user_gmail_address.to_string(),
        password.to_string(),
    );

    let mailer = SmtpTransport::relay(smtp_server)
        .unwrap()
        .credentials(credentials)
        .build();

    let result = mailer.send(&email);

    if result.is_ok() {
        Ok(())
    } else {
        Err(format!("Could not send mail: {:?}", result))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    #[ignore]
    fn sendmail() {
        // TODO: テストを書く
        // メール送信の良いテストの書き方が思いつかないので一旦保留にする（言い訳）
        // テスト記述の参考情報
        // @see: https://github.com/lettre/lettre/tree/master/tests
        // @see: https://medium.com/@11Takanori/simple-mocking-in-rust-3a21f1fa7e0c
    }

}