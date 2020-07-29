use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;

pub fn cli_clap<'a>() -> ArgMatches<'a> {
    let application = App::new("octooff")
        .about("Parses an input file to do awesome things")
        .version("0.0.1")
        .author("Owen Synge <osynge@googlemail.com>")
        .arg(
            Arg::with_name("verbose")
                .help("Increase log output.")
                .short("v")
                .multiple(true)
                .long("verbose"),
        )
        .arg(
            Arg::with_name("quiet")
                .help("Decrease log output.")
                .short("q")
                .multiple(true)
                .long("quiet"),
        )
        .subcommand(
            SubCommand::with_name("light")
                .about("light features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(Arg::with_name("list").long("list").help("list all lights")),
        )
        .subcommand(
            SubCommand::with_name("sensor")
                .about("light features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(Arg::with_name("list").long("list").help("list all sensors"))
                .arg(
                    Arg::with_name("watch")
                        .long("watch")
                        .help("watch all sensors"),
                ),
        );
    let matches = application.get_matches();
    return matches;
}
