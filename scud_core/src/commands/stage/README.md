# Stage

Will stage all files in the current local repository
regardless of whether or not you're currently in the repo's
root directory (Git-specific).

NOTE: For Mercurial, must be run from the project's root directory.

## Usage

### Default Usage

```
scud stage
```

### Usage with alias

```
scud s
```

## Example

```
scud stage
```

Output:
```
Repository status:

A  Cargo.lock
M  Cargo.toml
A  README.md
A  src/cli.rs
A  src/commands.rs
A  src/commands/commit/README.md
A  src/commands/parser.rs
A  src/commands/stage.rs
A  src/commands/stage/README.md
A  src/commands/stage/logging.rs
A  src/commands/stage/stage.rs
A  src/commands/unstage.rs
A  src/commands/unstage/README.md
A  src/commands/unstage/logging.rs
A  src/commands/unstage/unstage.rs
A  src/logging.rs
A  src/logging/general.rs
M  src/main.rs

💥 Done in 19ms.
```

### Git-specific Example

Repository structure:

```
/
|- src/ <-- current working directory
|- target/
|- tests/
|- .gitignore
|- Cargo.lock
|- Cargo.toml
|- README.md
```

The current working directory is the `src` directory. The command `scud stage` will stage all files in the current local repository, not just the files in the current working directory, `src`.

## Under the hood

### Git

-> `git add -A`

### Mercurial SCM

-> `hg add .`
