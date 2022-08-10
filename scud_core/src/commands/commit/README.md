# Commit

Commit all staged changes to the local repository. By default, scud uses the [Conventional Commit Specification](https://www.conventionalcommits.org/en/v1.0.0/) to generate commit messages. Additionally, scud comes with options for the [Angular Conventions](https://github.com/angular/angular/blob/22b96b9/CONTRIBUTING.md#-commit-message-guidelines) for commit messages, as well as an option to not use a commit standard. All of these options may be overridden by providing a custom message.

NOTE:

TODO: add potential notes here if needed

## Usage

### Default Usage

```
scud commit
```

### Usage with alias

```
scud c
```

## Example

```
scud commit
```

## Under the hood

### Git

-> `latest_commit = git log -1 --format=%H"`

-> `git reset latest_commit`

### Mercurial SCM

-> `hg revert --all`
