use std::process;

use clap::{crate_name, crate_version, crate_authors, App, Arg, SubCommand, ArgMatches};

fn pull(sub_m: &ArgMatches) {

}

fn main() {
    let mut app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("OCI image fetch CLI");
    let app_matches = &app
        .clone()
        .subcommand(
            SubCommand::with_name("pull")
                .version(crate_version!())
                .about("run cromwell container")
                .arg(
                    Arg::with_name("image_name")
                        .long("name")
                        .short("n")
                        .help("Specify image name")
                        .value_name("IMAGE")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("dir_name")
                        .short("o")
                        .value_name("NAME")
                        .help("Specify put dir name")
                        .takes_value(true),
                ),
        )
        .get_matches();

     match &app_matches.subcommand() {
        ("pull", Some(sub_m)) => pull(&sub_m),
        _ => {
            eprintln!("Unexpected arguments");
            app.print_help().unwrap();
            println!();
            process::exit(1);
        }
    }

}
