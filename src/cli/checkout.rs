use clap::Parser;
use std::process::{Command, exit};

use crate::common;

#[derive(Parser, Debug, Default)]
pub struct Args {
    // name of the tag
    #[arg(help = "Name of the tag")]
    tag: String,
}

pub fn execute(args: Args) {
    let repo = common::get_araki_git_repo().unwrap_or_else(|err| {
        eprintln!("Couldn't recognize the araki repo: {err}");
        exit(1);
    });

    let tag = args.tag;

    let git_ref = if tag == "latest" {
        match repo.find_reference("refs/heads/main") {
            Ok(res ) => res,
            Err(_err) => {
                eprintln!("Unable to find the latest commit at refs/heads/main");
                exit(1);
            }
        }
    } else {
        match repo.find_reference(&format!("refs/tags/{}", tag)) {
            Ok(res ) => res,
            Err(_err) => {
                eprintln!("{}", format!("Could not find tag '{}'", tag));
                exit(1);
            }
        }
    };

    let git_ref_object = git_ref.peel(git2::ObjectType::Commit).unwrap();
    let commit = git_ref_object
        .as_commit()
        .ok_or_else(|| git2::Error::from_str("Tag did not peel to a commit"))
        .unwrap();
    repo.checkout_tree(commit.as_object(), None)
        .expect("Unable to checkout tag");
    repo.set_head_detached(commit.id())
        .expect("Unable to set head");

    let _ = Command::new("pixi")
        .arg("install")
        .output()
        .expect("Failed to execute command");
}
