use clap::Parser;

#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub async fn execute() -> eyre::Result<()> {
        todo!()
    }
}
