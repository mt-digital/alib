
use std::process::Command;
use anyhow::anyhow;


/// Helper to construct a github URL to the repository
///
/// ```
/// assert_eq!(
///     "https://github.com/subtletea-research/ahelper-rstudio.git",
///     repo_url("subtletea-research/ahelper-rstudio")
/// )
/// ```
pub fn repo_url(repo_slug: &str) -> String {
    format!("https://github.com/{repo_slug}.git")
}


pub fn clone(repo_slug: &str) -> anyhow::Result<()> {

    let (_, _name) = repo_slug
        .split_once('/')
        .ok_or_else(|| anyhow!("Invalid repo slug"))?; 

    Command::new("git")
    .args([
        "clone",
        &repo_url(repo_slug),
        &_name,
    ])

    .status()?;

    Ok(())
}
    
