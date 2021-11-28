use libdmd::utils::config::builder::*;

fn main() {
    let config = ConfigBuilder::new()
        .builder()
        .project("dev")
        .dir(
            DirectoryBuilder::new()
                .builder()
                .name("config")
                .recursive(true)
                .file(
                    FileBuilder::new()
                        .builder()
                        .name("config")
                        .format(FileFormat::TOML),
                )
        )
        .build();
    match config {
        Ok(_) => println!("Ok"),
        Err(e) => println!("Error: {}", e),
    }
}
