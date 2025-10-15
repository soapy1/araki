use clap::{Parser, Subcommand};

/// Manage and share environments
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Initialize an environment
    Init {
        // name of the environment
        name: String,
    },
    // Activate an environment
    Activate {
        // name of the environment
        name: String,
    },
    // Add a package to an environment
    Add {
        // name of the environment, defaults to the current active environment
        #[arg(short, long)]
        name: Option<String>,
        // names of the packages
        #[arg(short, long, required = true, value_name = "SPEC")]
        specs: Vec<String>,
    },
    // Save a checkpoint for the environment
    Save {
        // name of the environment, defaults to the current active environment
        #[arg(short, long)]
        name: Option<String>,
        // name of the tag
        #[arg(short, long, required = true)]
        tag: Vec<String>, 
    },
    // List all available environments
    List {

    },
    // Install a tag into an environment
    Install {
        // name of the environment, defaults to the current active environment
        #[arg(short, long)]
        name: Option<String>,
        // name of the tag to install
        tag: String
    },
    // Push environment to a remote repo
    Push {
        // name of the tag to push
        tag: String
    },
    // Pull environment from a remote repo
    Pull {
        // name of the tag to push
        tag: String
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { name }) => {
           println!("(not) initializing env: {}", name); 
        }
        Some(Commands::Activate { name }) => {
            println!("(not) activating env: {}", name); 
        }
        Some(Commands::Add { name, specs }) => {
            if let Some(n) = name {
                println!("(not) adding specs to '{}': {:?}", n, specs);
            } else {
                println!("(not) adding specs to current environment: {:?}", specs);
            }
        }
        Some(Commands::Save { name, tag }) => {
            if let Some(n) = name {
                println!("(not) saving env '{}': {:?}", n, tag);
            } else {
                println!("(not) adding specs to current environment: {:?}", tag);
            }
        }
        Some(Commands::List {  }) => {
           println!("(not) listing environments"); 
        }
        Some(Commands::Install { name, tag }) => {
            if let Some(n) = name {
                println!("(not) installing env '{}': {:?}", n, tag);
            } else {
                println!("(not) installing to current environment: {:?}", tag);
            }
        }
        Some(Commands::Push { tag }) => {
           println!("(not) pushing env: {}", tag); 
        }
        Some(Commands::Pull { tag }) => {
           println!("(not) pulling env: {}", tag); 
        }
        None => {
            println!("No subcommand provided. Use --help for usage.");
        }
    }
}
