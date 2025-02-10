use anyhow::{self, Ok};
use zxcvbn::zxcvbn;
use clap::Parser;
use rcli::{process::process_genpass, process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Csv(opts) => {
            let output=if let Some(output) = opts.output {
                output.clone()
            } else {
                let fmt:&'static str= opts.format.into();
                format!("output.{}",fmt)
            };
            process_csv(&opts.input, output, opts.format)
        },
        SubCommand::GenPass(opts) => {
            let password = process_genpass(opts.length, opts.uppercase, opts.lowercase, opts.number, opts.symbols)?;
            let estimate = zxcvbn(&password, &[]);
            eprintln!("密码：{}，等级：{}", password,estimate.score());
            Ok(())
        }
    }
}
