use clap::Parser;
use color_eyre::eyre::Result;
use license_fetcher::get_package_list_macro;
use tracing::info;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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

#[cfg(test)]
mod test {
    use color_eyre::eyre::Ok;
    use pretty_assertions::assert_eq;

    use crate::setup::setup;

    use super::*;

    #[test]
    fn example_test() -> Result<()> {
        let _guard = setup();

        assert_eq!(2 + 2, 4);

        Ok(())
    }
}
