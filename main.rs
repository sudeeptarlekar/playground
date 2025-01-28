use anyhow::Result;

fn main() {
    println!("Hello, World!");
    println!("Welcome to Rust world!");
}

fn fetch_remote() -> Result<()> {
    let repo = git2::Repository::open(".")?;
    Ok(())
}
