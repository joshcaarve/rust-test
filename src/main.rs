extern crate clap;
use clap::{App, Arg, SubCommand};


#[path = "./get/crypto.rs"] mod crypto;


fn main() {
    let matches = App::new("portfolio")
        .version("0.0.1")
        .author("Joshua B")
        .about("WIP")
        .subcommand(
            SubCommand::with_name("get")
                .about("get a portfolio")
                .version("0.0.1")
                .author("Joshua B")
                .arg(
                    Arg::with_name("crypto")
                        .short("c")
                        .long("crypto")
                        .help("get crypto balance"),
                )
                .arg(
                    Arg::with_name("stock")
                        .short("s")
                        .long("stock")
                        .help("get stock balance"),
                ),
        )
        .subcommand(
            SubCommand::with_name("export")
                .about("export a portfolio")
                .version("0.0.1")
                .author("Joshua B")
                .arg(
                    Arg::with_name("crypto")
                        .short("c")
                        .long("crypto")
                        .help("get crypto balance"),
                )
                .arg(
                    Arg::with_name("stock")
                        .short("s")
                        .long("stock")
                        .help("get stock balance"),
                ),
        )
        .get_matches();

    if matches.is_present("get") {
        println!("get command")
    }
    crypto::get_crypto();
}
