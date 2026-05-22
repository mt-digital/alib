use std::fs;
use std::process::Command;
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use dirs::home_dir;

use crate::cmd;

use crate::github;


pub enum AHelper {
    Add {
        module_repo_slug: String,
        module_name: String, 
    },

    Run {
        module_name: String, 
    },

    List,

    Help,
}



impl cmd::Command for AHelper {


    fn parse(args: &[String]) -> Result<Self> {

        match args {

            [_, cmd, value] => {

                match cmd.as_ref() {

                    "add" => { 
                         
                        Ok(
                            AHelper::Add{ 
                                module_repo_slug: value.clone(),
                                module_name: module_name(&value)?
                            }
                        )
                    }

                    "run" => { 
                        Ok(
                            AHelper::Run { 
                                module_name: value.clone() 
                            }
                        ) 
                    }

                    _ => Err(anyhow!("Dang")),
                }
            }

            
            // If there are only two arguments, must be list or check
            [_, cmd] => {

                match cmd.as_str() {


                    "list" => Ok(Self::List),


                    "help" => Ok(Self::Help),


                    _ => Err(anyhow!("Command {cmd} not recognized")),
                }
            }

            _ => Err(anyhow!(help())),
        }

    }


    fn go (self: Self) -> Result<()> {

        match self {

            AHelper::Add { module_name, module_repo_slug } => {

                github::clone(&module_repo_slug)?;

                println!("Added {module_name} from {module_repo_slug} on GitHub.");

                println!(
                        "Module code was cloned to {}", 
                        modules_dir()
                            .join(module_name)
                            .display()
                );

                Ok(())
            }

            AHelper::Run { module_name } => {

                Module{ name: module_name }.run()?;
                Ok(())
            }

            AHelper::List => {

                // Iterate through directories in modules directory...
                for entry in fs::read_dir(modules_dir())? {
                    // ...and print each one
                    println!(
                        "{}", 
                        entry?
                            .file_name()
                            .to_string_lossy()
                            .into_owned()
                    );
                }

                Ok(())

            }

            // print help message 
            AHelper::Help => {
                printhelp();
                Ok(())
            }
        }
    }
}


/// ahelper uses a module system, adding, running, and listing modules
pub struct Module {
    pub name: String,
}

impl Module {

    pub fn run(self: Self) -> Result<()> {

        // Run the module's main.sh script, collecting the status if successful
        let status = Command::new("bash")
            .arg(module_script(&self.name))
            .status()?;

        if status.success() {
            return Ok(());
        }

        Err(anyhow!(
            "Module '{}' exited with failure",
            self.name,
        ))
    }

}


/// Get the module name from a github slug
///
/// ```
/// use crate::ahelper::module_name;
///
/// assert_eq!(
///     "ahelper-rstudio",
///     module_name("subtletea-research/ahelper-rstudio")
/// )
/// ```
pub fn module_name(repo_slug: &str) -> Result<String> {

    let parts = repo_slug
        .split_once("/")
        .ok_or_else(|| anyhow!("Invalid repo slug"))?;

    Ok(parts.1.to_string())
}


pub fn ahelper_home() -> PathBuf {
    home_dir()
        .expect("Could not find home directory")
        .join(".local/share/ahelper")
}

pub fn modules_dir() -> PathBuf {
    ahelper_home().join("modules")
}

pub fn module_dir(module_name: &str) -> PathBuf {
    modules_dir().join(module_name)
}

pub fn module_script(module_name: &str) -> PathBuf {
    module_dir(module_name).join("main.sh")
}

fn printhelp() { eprint!("{}", help()) }


fn help() -> String {

    r#"
ahelper

usage:
    ahelper add <repo-slug>
    ahelper run <module>
    ahelper list
    ahelper check
"#
    .to_string()
}
