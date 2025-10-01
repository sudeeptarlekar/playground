use anyhow::{Context, Result};
use git2::{Remote, Repository};

fn main() {
    println!("Hello, world!");
    println!("Lorem Ipsum.....");
    println!("Rim info file should start with {}", dictionary::RIM_INFO);
    println!("Rim info file should start with {}", dictionary::RIM_INFO);
    println!("Rim info file should start with {}", dictionary::RIM_INFO);
    println!("Rim info file should start with {}", dictionary::RIM_INFO);
    println!("Rim info file should start with {}", dictionary::RIM_INFO);
    println!("Rim info file should start with {}", dictionary::RIM_INFO);
}

fn fetch_remote<'repo>(repo: &'repo Repository, url: &str) -> Result<Remote<'repo>> {
    repo.remote_anonymous(url)
        .context("Unable to create an anonymous remote from url")
}
