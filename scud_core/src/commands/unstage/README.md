# Unstage

Will unstage any staged files in the current working directory.

NOTE:

- Git
  - can be run from any nested directory within the local repository, not necessarily the root.
- Mercurial SCM
  - must be run from the project's root directory.

## Usage

### Default Usage

```
scud unstage
```

### Usage with alias

```
scud u
```

## Example

```
scud unstage
```

Output:

```
Unstaged changes after reset:
M       Cargo.toml
M       src/main.rs
```

<span style="color:yellow">` Repository status:`</span>

<span style="color:red">` M Cargo.toml`</span>

<span style="color:red">`?? Cargo.lock`</span>

<span style="color:red">`?? README.md`</span>

<span style="color:red">`?? src/cli.rs`</span>

<span style="color:red">`?? src/commands.rs`</span>

<span style="color:red">`?? src/commands/`</span>

<span style="color:red">`?? src/logging.rs`</span>

<span style="color:red">`?? src/logging/`</span>

```💥 Done in 24ms.```

## Under the hood

### Git

-> `latest_commit = git log -1 --format=%H"`

-> `git reset latest_commit`

### Mercurial SCM

-> `hg revert --all`
```
