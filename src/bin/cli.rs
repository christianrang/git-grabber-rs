use git2::{Cred, RemoteCallbacks, Repository};
use git_grabber::git_grabber;
use resolve_path::PathResolveExt;
use std::env;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let home = env::var("HOME")?;

    let configuration_path_string = format!("{}/.config/git-grabber/config.yaml", home);
    let configuration_path = Path::new(&configuration_path_string);
    let configuration = git_grabber::config::get_configuration(configuration_path)?;

    for repository in &configuration.git.repositories {
        let target_directory = repository.target_directory.resolve();
        let repo = match Repository::open(&target_directory) {
            Ok(repo) => repo,
            Err(_) => {
                let mut callbacks = RemoteCallbacks::new();
                callbacks.credentials(|_url, username_from_url, _allowed_types| {
                    Cred::ssh_key(
                        username_from_url.expect("no username provided in the url"),
                        None,
                        Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
                        None,
                    )
                });
                let mut fetch_options = git2::FetchOptions::new();
                fetch_options.remote_callbacks(callbacks);

                let mut builder = git2::build::RepoBuilder::new();
                builder.fetch_options(fetch_options);
                builder.bare(true);
                builder.clone(&repository.ssh_url, &target_directory)?
            }
        };

        for branch in &repository.branches {
            let target_dir_buf = target_directory.join(branch);
            let target_directory = target_dir_buf.as_path();

            match repo.find_reference(&format!("refs/heads/{}", branch)) {
                Ok(reference) => {
                    let mut worktree_add_options = git2::WorktreeAddOptions::new();
                    worktree_add_options.reference(Some(&reference));

                    let worktree = repo.worktree(&branch, target_directory, Some(&worktree_add_options));
                    match worktree {
                        Ok(worktree) => worktree,
                        Err(_) => {
                            repo.find_worktree(&branch)?
                        }
                    };

                    continue;
                }
                // TODO(crang): determine the type of error that fails when the reference is found
                // and handle the other errors while ignoring that one.
                Err(_) => {}
            };

            let worktree = repo.worktree(&branch, target_directory, None);
            match worktree {
                Ok(worktree) => worktree,
                Err(_) => {
                    repo.find_worktree(&branch)?
                }
            };
        }
    }

    return Ok(());
}
