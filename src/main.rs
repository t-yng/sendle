extern crate kindle_push;

use clap::Clap;
// use kindle_push::mailer::sendmail;

// driveの使い方の例
// WARNING: clapのREADME.mdにあるderiveのexampleは正しくないので注意
// @see: https://github.com/clap-rs/clap/blob/master/clap_derive/examples
#[derive(Clap, Debug)]
struct Opt {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    /// push pdf file to your kindle
    Push(Push),
}

#[derive(Clap, Debug)]
struct Push {
    /// pdf files to push
    #[clap(required=true, min_values=1)]
    input: Vec<String>,
}

fn main() {
    let opt = Opt::parse();
    match opt.subcommand {
        SubCommand::Push(args) => {
            println!("send {:?} to your kindle", args.input.join(", "));
        }
    }
}
