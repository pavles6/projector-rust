use anyhow::Result;
use clap::Parser;
use projector_rust::{
    config::{Config, Operation},
    opts::Opts,
    projector::Projector,
};

fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into()?;

    let mut proj = Projector::from_config(config.config, config.pwd);

    match config.operation {
        Operation::Print(None) => {
            let values = proj.get_values();
            let value = serde_json::to_string(&values)?;

            println!("{}", value);
        }
        Operation::Print(Some(k)) => {
            proj.get_value(&k).map(|v| println!("{}", v));
        }
        Operation::Add(k, v) => {
            proj.set_value(k, v);
            proj.save()?;
        }
        Operation::Remove(k) => {
            proj.remove_value(&k);
            proj.save()?;
        }
    }

    return Ok(());
}
