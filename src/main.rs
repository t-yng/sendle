extern crate sendle;

use clap::Clap;
use sendle::subcommand;
use sendle::config;

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
    Config(Config)
}

#[derive(Clap, Debug)]
struct Send {
    /// pdf files to send
    #[clap(required=true, min_values=1)]
    input: Vec<String>,
}

#[derive(Clap, Debug)]
struct Config {}

fn main() {
    let opt = Opt::parse();
    let file = config::ConfigFile::new();
    match opt.subcommand {
        SubCommand::Send(args) => {
            let config = config::Config::load(&file);
            println!("sending pdf files ...");
            match subcommand::send(args.input, &config) {
                Ok(()) => println!("succeeded send pdf files."),
                Err(err) => println!("failed send pdf files:\n{}", err),
            };
        },
        SubCommand::Config(_) => {
            match subcommand::config(&file) {
                Ok(()) => println!("create config file {}", file.path_str()),
                Err(err) => println!("failed create config file: \n{}", err.to_string()),
            }
        }
    }
}
