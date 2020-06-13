use std::path::Path;
use std::fs;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{SinglePart, MultiPart, header };
use lettre::transport::smtp::{authentication::Credentials};
use crate::config::{Config};

struct Email {
    pub to_address: String,
    pub smtp_credential:  SmtpCredential,
    pub file:  String,
}

struct SmtpCredential {
    pub user_gmail_address: String,
    pub password: String,
}

// TODO: send multiple files
pub fn send(files: &Vec<String>, config: &Config) -> Result<(), String> {
    let kindle = &config.kindles[0];

    let smtp_credential = SmtpCredential {
        user_gmail_address: config.credential.user_gmail_address.clone(),
        password: config.credential.google_application_password.clone(),
    };

    let mail = Email {
        to_address: kindle.mail_address.clone(),
        smtp_credential,
        file: files[0].clone(),
    };

    println!("sending: {}", files[0]);

    sendmail(&mail)
}

// TODO: 送信先のKindel端末のメールアドレスを指定する
// TODO: ユーザーのメールアドレス（設定ファイルから取得）
// TODO: ユーザーのパスワード（設定ファイルから取得）
fn sendmail(mail: &Email) -> Result<(), String> {
    let smtp_server = "smtp.googlemail.com";

    // REFACTOR: PDFの処理はモジュールを分けて引数で受け取るようにする
    // pdfファイルを読み込み
    let path = Path::new(mail.file.as_str());
    let file_name = match path.file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => return Err(format!("Could not get filename: {}", mail.file)),
    };
    let pdf = fs::read(&mail.file).unwrap();

    // TODO: pdfファイルが複数の場合を考慮する
    // lettreの実装example
    // @see: https://github.com/lettre/lettre/blob/master/src/message/mod.rs
    let email = Message::builder()
        .to(mail.to_address.parse().unwrap())
        .from(mail.smtp_credential.user_gmail_address.parse().unwrap())
        .subject("send some files to my kindle by kindle-push")
        .multipart(
            MultiPart::mixed()
            .singlepart(
                SinglePart::builder()
                .header(header::ContentType(
                    "text/plain; charset=utf8".parse().unwrap()
                ))
                .body("")
            )
            .singlepart(
                SinglePart::base64()
                .header(header::ContentType(format!("application/pdf; name=\"{}\"", file_name).parse().unwrap()))
                .header(header::ContentDisposition {
                    disposition: header::DispositionType::Attachment,
                    parameters: vec![
                        header::DispositionParam::Filename(
                            header::Charset::Ext("utf-8".into()),
                            None, file_name.as_bytes().into()
                        )
                    ]
                })
                .body(pdf)
            )
        )
        .unwrap();

    let credentials = Credentials::new(
        mail.smtp_credential.user_gmail_address.to_string(),
        mail.smtp_credential.password.to_string(),
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

mod test {
    use super::*;

    // TODO: 設定ファイルをスタブ化する
    // テストこけるので一旦無視する
    #[test]
    #[ignore]
    fn test_send() {
        // 環境変数の値を使って実際にメールを送信して成功するかテストしている
        // TODO: ファイル添付が正常にできていることをテストしたい
        // テスト記述の参考情報
        // ローカルでSMTPサーバー起動する？
        // @see: https://github.com/lettre/lettre/tree/master/tests
        // @see: https://medium.com/@11Takanori/simple-mocking-in-rust-3a21f1fa7e0c
        // let files = vec![String::from("linux.pdf")];
        // let result = send(files);
        // assert_eq!(result, Ok(()));
    }
}