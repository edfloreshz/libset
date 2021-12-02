use anyhow::Result;
use libdmd::utils::conf::config::*;

fn main() -> Result<()> {
    let _config = Config::new()
        .name("devmode")
        .author("Eduardo Flores")
        .about("Development management app.")
        .version("0.1.1")
        .add(Element::new("config")
            .child(Element::new("config.toml").format(Format::File))
            .child(Element::new("user")
                .child(Element::new("user.toml").format(Format::File))
                .child(Element::new("edfloreshz")
                    .child(Element::new("edfloreshz.toml").format(Format::File))
                )
            )
        )
        .add(Element::new("logs"))
        .add(Element::new("paths").child(Element::new("devpaths").format(Format::File)))
        .write()?;
    // println!("{:#?}", config);
    Ok(())
}

mod old {
    use anyhow::Result;
    use libdmd::utils::config::config::Config;
    use libdmd::utils::config::directory::Directory;
    use libdmd::utils::config::file::File;
    use libdmd::utils::config::format::FileFormat;

    fn _main() -> Result<()> {
        let config = Config::new()
            .project("devmode")
            .dir(
                Directory::new()
                    .name("config")
                    .file(File::new().name("config").format(FileFormat::TOML)),
            )
            .dir(Directory::new().name("logs"))
            .dir(
                Directory::new()
                    .name("paths")
                    .file(File::new().name("devpaths").format(FileFormat::TOML)),
            );
        println!("{:#?}", config);
        Ok(())
    }
}
