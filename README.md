# sendle
![CI](https://github.com/t-yng/sendle/workflows/CI/badge.svg)

This is cli tool to send pdf files to your kindle.

```sh
$ sendle send linux.pdf
```

## Installation

### Homebrew

```sh
$ brew tap t-yng/sendle
$ brew install t-yng/sendle/sendle
```

## How to use

### Setup

Sendle uses gmail smtp server to send email to your kindle.  
You need to set credentials for gmail smtp server and kindle info you will send pdf files.

| Key                         | Default | Description |
|-----------------------------|---------|-------------|
| gmail_address               | -       | Your gmail address that will be used send email to kindles. |
| google_application_password | -       | Application password to certify smtp server.<br>Please read below how to get your application password. <br> [Google App Passwords](https://myaccount.google.com/apppasswords) |
| kindle_name                 | default | Kindle name to be managed by Sendle. <br> Any Value is Ok. |
| kindle_mail_address         | -       | Send-to-Kindle-email address. <br> Please read below, how to get the mail address. <br> [Send to Kindle by E-mail](https://www.amazon.com/gp/sendtokindle/email) |

```sh
$ sendle config
gmail_address: example@gmail.com
google_application_password: expfldodhykovwjf
kindle_name: paper_white
kindle_mail_address: example_K4HblsQ4@gmail.com
```

### Send pdf files

```sh
$ sendle send linux.pdf
```

## Develop

### Unit Test

```sh
$ cargo test
```
