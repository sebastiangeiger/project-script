#`project` script suite [![Build Status](https://travis-ci.org/sebastiangeiger/project-script.svg?branch=master)](https://travis-ci.org/sebastiangeiger/project-script)

The `project` suite simplifies common workflows that I use on almost every project.
This tool might not necessarily be for you but I hope you can draw some inspiration from it.

## Configuration
All configuration and state is kept in the `~/.projects` file.
The `~/.projects` file is storing the data in JSON.
I am assuming you are keeping the `~/.projects` file in sync using Dropbox or Owncloud.

## Options
### `--config=<FILE>`
Will use FILE instead of `~/.projects` as source for configuration
### `--dry-run`
You can preview what changes a command would do by adding `--dry-run`.
This will print out the changes instead of executing them.

## Subcommands

### `project-list-pull`

Read the `~/.projects` file, clone new projects in git or setup git remotes as necessary.
This command fails if any of the git commands fails if:

  * a folder is already taken and does not have the same git remotes
  * any of the underlying git commands fails
  * git remotes with the same name exist but point to different urls

### `project-list-push`

Update the `~/.projects` file to reflect the state of the current machine.
It will only create new git remotes, never delete them.
This command fails if:

  * a folder with the same name already exists but points to a different git remote.


### `project-list-sync`

This will do a `project-list-pull` followed by a `project-list-push`.
