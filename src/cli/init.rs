use clap::Parser;
use std::fs;

use crate::cli::common;

#[derive(Parser, Debug, Default)]
pub struct Args {
    // name of the environment
    #[arg(help="Name of the environment")]
    name: String,
}

pub fn execute(args: Args){
    println!("initializing env: {}", args.name);
    
    let Some(akari_envs_dir) = common::get_default_akari_envs_dir()
    else {
        println!("error!");
        return
    };

    let project_env_dir = akari_envs_dir.join(args.name);
    let _ = fs::create_dir_all(project_env_dir);
}