extern crate clap;
use clap::{App, Arg};

pub fn get_command_line() -> bool {
    let matches = App::new("Pillars Game for Gnome")
        .version("v0.1")
        .author("John Ward")
        .about("A game of mathing 3 pieces in a vertical row")
        .arg(
            Arg::with_name("DISABLE_GL")
                .short("d")
                .long("disable-gl")
                .help("Doesn't hardware acceleration"),
        )
        .get_matches();

    matches.is_present("DISABLE_GL")
}
