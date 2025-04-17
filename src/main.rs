use clap::Parser;
use color_eyre::eyre::Result;
use license_fetcher::get_package_list_macro;
use tracing::info;

mod arguments;
mod setup;

fn main() -> Result<()> {
    let _guard = setup::setup();

    let arguments = arguments::Arguments::parse();

    if arguments.license {
        let packages = get_package_list_macro!().unwrap();
        println!("{}", packages);
        return Ok(());
    }

    info!("Hello, world!");

    Ok(())
}
