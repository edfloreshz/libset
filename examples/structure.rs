use anyhow::Result;
use libset::project::Project;

fn main() -> Result<()> {
    Project::new("com", "organization", "App")
        .author("Eduardo Flores")
        .about("Example app.")
        .version("0.1.1");
    Ok(())
}
