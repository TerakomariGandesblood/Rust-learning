// CLI (command line interface)

use std::{fs, path::PathBuf, thread, time::Duration};

use anyhow::{Context, Ok, Result};
use clap::Parser;
use indicatif::ProgressBar;
use tracing::{debug, info, trace};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf,
}

fn find_matches(content: &str, pattern: &str) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}

fn main() -> Result<()> {
    // 应该在 main 函数中使用，遇到错误时，将打印错误信息并退出
    let args = Cli::parse();

    let pb = ProgressBar::new(5);
    for i in 0..5 {
        thread::sleep(Duration::from_millis(200));
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    tracing_subscriber::fmt::init();
    let number_of_yaks = 3;
    info!("preparing to shave yaks: {}", number_of_yaks);

    debug!("debug");
    trace!("trace");

    // TODO use https://doc.rust-lang.org/std/io/struct.BufReader.html#examples
    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.to_str().unwrap()))?;

    find_matches(&content, &args.pattern);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_a_match() {
        find_matches("lorem ipsum\ndolor sit amet", "lorem");
        // assert_eq!()
    }
}
