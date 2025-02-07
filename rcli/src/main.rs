use anyhow;
use clap::Parser;
use rcli::opt::{Opts, SubCommand};
use rcli::process::process_csv;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output),
    }
}
