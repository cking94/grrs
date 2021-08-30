/// Custom implementation of 'grep' - 'grrs' (grass)
/// Searches for string in file
/// Outputs matched line
use anyhow::{Context, Result};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    /// Pattern to look for
    pattern: String,
    /// Path to the file to search
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file: `{:?}`", &args.path))?;
    let content = { content };
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
