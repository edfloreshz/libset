use anyhow::Result;
use libdmd::utils::config::*;

fn main() -> Result<()> {
    let _config = Config::new()
        .name("devmode")
        .author("Eduardo Flores")
        .about("Development management app.")
        .version("0.1.1")
        .add(
            Element::new("config")
                .child(Element::new("config.toml").format(Format::File))
                .child(
                    Element::new("user")
                        .child(Element::new("user.toml").format(Format::File))
                        .child(
                            Element::new("edfloreshz")
                                .child(Element::new("edfloreshz.toml").format(Format::File)),
                        ),
                ),
        )
        .add(Element::new("logs"))
        .add(Element::new("paths").child(Element::new("devpaths").format(Format::File)))
        .write()?;
    let current = Config::current();
    match current {
        None => println!("Could not get current configuration"),
        Some(current) => println!("{:#?}", current)
    }
    Ok(())
}
