use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser, help = "Path to the configuration file")]
    pub config: String,

    #[clap(
        long,
        value_parser,
        help = "Optional home directory path. Defaults to home (~/)"
    )]
    pub home: Option<String>,
}
