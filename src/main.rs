#[macro_use]
extern crate serde_derive;

use std::process;

use clap::{crate_authors, crate_name, crate_version, App, Arg, ArgMatches, SubCommand};

mod image;

use self::image::Image;

fn do_pull(sub_m: &ArgMatches) {
    let mut image = Image::new(sub_m.value_of("image_name").expect("specify image name"));
    let path = sub_m.value_of("dir_name").unwrap_or("./hoge");
    image.pull(path).expect("failed fetch oci image");
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
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    match &app_matches.subcommand() {
        ("pull", Some(sub_m)) => do_pull(&sub_m),
        _ => {
            eprintln!("Unexpected arguments");
            app.print_help().unwrap();
            println!();
            process::exit(1);
        }
    }
}
