use anyhow;
use clap::Parser;
use rcli::{
    cli::{Opts, SubCommand},
    process::{process_decode, process_encode, process_genpass},
    process_csv, Base64SubCommand,
};
use zxcvbn::zxcvbn;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                let fmt: &'static str = opts.format.into();
                format!("output.{}", fmt)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbols,
            )?;
            let estimate = zxcvbn(&password, &[]);
            eprintln!("密码：{}，等级：{}", password, estimate.score());
        }
        SubCommand::Base64(cmd) => match cmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
    }
    anyhow::Ok(())
}
