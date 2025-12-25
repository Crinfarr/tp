use clap::Parser;
mod daemon;
mod errors;
mod socket;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[arg(long)]
    daemon: bool,
}

#[tokio::main]
async fn main() -> Result<(), crate::errors::AppError> {
    let args = CliArgs::parse();
    if args.daemon {
        crate::daemon::run().await?;
        return Ok(());
    }
    Ok(())
}
