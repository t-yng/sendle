extern crate kindle_push;

use clap::Clap;
use kindle_push::subcommand::send;

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
    /// send pdf file to your kindle
    Send(Send),
}

#[derive(Clap, Debug)]
struct Send {
    /// pdf files to send
    #[clap(required=true, min_values=1)]
    input: Vec<String>,
}

fn main() {
    let opt = Opt::parse();
    match opt.subcommand {
        SubCommand::Send(args) => {
            println!("sending pdf files ...");
            match send(args.input) {
                Ok(()) => println!("succeeded send pdf files."),
                Err(err) => println!("failed send pdf files:\n{}", err),
            };
        }
    }
}
