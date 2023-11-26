# Git Grabber

This project was built to solve a problem of keeping many microservice repos local and up-to-date.

It is in the early stages, but the goal is to have a runner or a cronjob that allows updates on an interval to keep main branches up to date.
The project will include protections (where possible) to prevent changes to the main branch, so it can be updated without intervention.
It will support other branches outside of the main branch to have this protection.

## Installation
```bash
git clone https://github.com/christianrang/git-grabber-rs.git git-grabber-rs
cd git-grabber-rs
cargo install
```

## Configuration

The configuration file path is `$HOME/.config/git-grabber/config.yaml`.

Example configuration:
```yaml
ssh:
  # Path to the SSH Key to use for authentication
  # This key should not have a password on it. A password will negate the automated nature of this project.
  path: "~/.ssh/id_rsa"

git:
  repos:
      # Target dir is the directory you would like the project to be stored in.
    - target_dir: "/tmp/test"
      # The ssh url for the repo.
      ssh_url: "git@github.com:christianrang/dotfiles.git"
      # These branches will be created and kept up to date. Currently, if there is a change in the worktree the program may error.
      branches:
        - main
    - target_dir: "/tmp/test1"
      ssh_url: "git@github.com:christianrang/rustplayground.git"
      branches:
        - main
```
