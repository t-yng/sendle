use colored::*;
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
            println!("sending pdf files");
            match subcommand::send(&args.input, &config) {
                Ok(()) => println!("\n{}", "Send pdf files complete!".bright_green().bold()),
                Err(err) => println!("{}:\n{}", "failed send pdf files".red(), err.red()),
            };
        },
        SubCommand::Config(_) => {
            match subcommand::config(&file) {
                Ok(()) => println!("\n{}: {}", "Created config file".bright_green().bold(), file.path_str()),
                Err(err) => println!("{}:\n{}", "failed create config file".red(), err.to_string().red()),
            }
        }
    }
}
