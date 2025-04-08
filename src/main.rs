use anyhow::Context;
use anyhow::Result;
use git2::Repository;

fn main() -> Result<()> {
    println!("Hello, world!");
    let repo =
        Repository::open(".").context("Current working directory is not a Git repository")?;
    Ok(())
}

fn some_random_function() {
    println!("Hello");
}
