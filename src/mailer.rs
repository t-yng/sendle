use std::path::Path;
use std::fs;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{SinglePart, header };
use lettre::transport::smtp::{authentication::Credentials};

// TODO: 送信先のKindel端末のメールアドレスを指定する
// TODO: ユーザーのメールアドレス（設定ファイルから取得）
// TODO: ユーザーのパスワード（設定ファイルから取得）
pub fn sendmail(to_address: &str, user_gmail_address: &str, password: &str, file: &str) -> Result<(), String> {
    let smtp_server = "smtp.googlemail.com";

    // REFACTOR: PDFの処理はモジュールを分けて引数で受け取るようにする
    // pdfファイルを読み込み
    let path = Path::new(file);
    let file_name = match path.file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => return Err(format!("Could not get filename: {}", file)),
    };
    let pdf = fs::read(file).unwrap();

    // TODO: pdfファイルが複数の場合を考慮する
    // lettreの実装example
    // @see: https://github.com/lettre/lettre/blob/master/src/message/mod.rs
    let email = Message::builder()
        .to(to_address.parse().unwrap())
        .from(user_gmail_address.parse().unwrap())
        .subject("send some files to my kindle by kindle-push")
        .singlepart(
            SinglePart::base64()
            .header(header::ContentType("application/pdf".parse().unwrap()))
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
    use super::*;

    #[test]
    #[ignore]
    fn test_sendmail() {
        // TODO: テストを書く
        // メール送信の良いテストの書き方が思いつかないので一旦保留にする（言い訳）
        // テスト記述の参考情報
        // ローカルでSMTPサーバー起動する？
        // @see: https://github.com/lettre/lettre/tree/master/tests
        // @see: https://medium.com/@11Takanori/simple-mocking-in-rust-3a21f1fa7e0c
        let to_address = "<to_addres>";
        let user_gmail_address = "<user_gmail_address>";
        let password = "<google_application_password>";
        let file = "<pdf_file_path>";

        let result = sendmail(to_address, user_gmail_address, password, file);
        assert_eq!(result, Ok(()));
    }

}