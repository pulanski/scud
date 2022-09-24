<div id="top"></div>

<!-- PROJECT SHIELDS -->

<p align="center">
    <a href="https://github.com/pulanski/scud">
        <img src="https://github.com/pulanski/scud/blob/main/assets/logo.png?raw=true" width="300">
    </a>
    <br>
    <!-- <a href="https://github.com/orhun/git-cliff/releases">
        <img src="https://img.shields.io/github/v/release/orhun/git-cliff?style=flat&labelColor=1C2C2E&color=C96329&logo=GitHub&logoColor=white">
    </a> -->
    <a href="https://crates.io/crates/scud/">
        <img src="https://img.shields.io/crates/v/scud?style=flat&labelColor=1C2C2E&color=C96329&logo=Rust&logoColor=white">
    </a>
    <a href="https://https://www.rust-lang.org/">
        <img src="https://img.shields.io/github/languages/top/pulanski/scud?style=flat&labelColor=1C2C2E&color=C96329&logo=Rust&logoColor=white">
    </a>
    <!-- <a href="https://codecov.io/gh/orhun/git-cliff">
        <img src="https://img.shields.io/codecov/c/gh/orhun/git-cliff?style=flat&labelColor=1C2C2E&color=C96329&logo=Codecov&logoColor=white">
    </a> -->
    <br>
    <!-- <a href="https://github.com/orhun/git-cliff/actions?query=workflow%3A%22Continuous+Integration%22">
        <img src="https://img.shields.io/github/workflow/status/orhun/git-cliff/Continuous%20Integration?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=GitHub%20Actions&logoColor=BEC5C9">
    </a> -->
    <!-- <a href="https://github.com/orhun/git-cliff/actions?query=workflow%3A%22Continuous+Deployment%22">
        <img src="https://img.shields.io/github/workflow/status/orhun/git-cliff/Continuous%20Deployment?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=GitHub%20Actions&logoColor=BEC5C9&label=deploy">
    </a> -->
    <!-- <a href="https://hub.docker.com/r/orhunp/git-cliff">
        <img src="https://img.shields.io/docker/cloud/build/orhunp/git-cliff?style=flat&labelColor=1C2C2E&color=BEC5C9&label=docker&logo=Docker&logoColor=BEC5C9">
    </a> -->
    <a href="https://docs.rs/scud_core/">
        <img src="https://img.shields.io/docsrs/scud_core?style=flat&color=BEC5C9&logo=Rust&logoColor=BEC5C9E">
    </a>
    <br>
    <a href="https://en.wikipedia.org/wiki/MIT_License">
        <img src="https://img.shields.io/github/license/pulanski/scud?labelColor=425C60&color=CFD6DD&">
    </a>
    <!-- <a href="https://crates.io/crates/scud/">
        <img src="https://img.shields.io/tokei/lines/github/pulanski/scud?style=flat&labelColor=425C60&color=CFD6DD&">
    </a> -->
</p>

# 💨 scud

### 🚧 WIP 🚧

_move fast because or as if driven by the wind._

<p align="center">
  <a href="#why-build-scud">Why?</a> |
  <a href="#goals">Goals</a> |
  <a href="#project-status">Status</a> |
  <a href="#getting-started">Getting started</a>
</p>

A tool for streamlining the many version control processes of your development workflow written with an emphasis on ease-of-use and expressive usage diagnostics.

<!-- -   🏎️ Fast (Rust-powered) -->
<!-- -   🧩 Modular -->
<!-- -   ⚙️ Configurable -->
<!-- -   ✨ Feature-rich -->
<!-- -   🍱 Sane defaults -->

<p align="center">
    <a href="https://github.com/pulanski/scud/issues">Report Bug</a>
    ·
    <a href="https://github.com/pulanski/scud/issues">Request Feature</a>
</p>

<!-- TABLE OF CONTENTS -->

<details>
  <summary>Table of Contents</summary>

-   [About](#about)
    -   [Why?](#why-build-scud)
    -   [Goals](#goals)
    -   [Status](#status)
-   [Getting started](#getting-started)
    -   [Prerequisites](#prerequisites)
    -   [Installation](#installation)
        -   [From crates.io](#from-cratesio)
        <!-- -   [Using pacman](#using-pacman)
        -   [Binary releases](#binary-releases)
        -   [Build from source](#build-from-source) -->
-   [Usage](#usage) - [Command Line Arguments](#command-line-arguments)
-   [Roadmap](#roadmap)
    <!-- -   [Examples](#examples) -->
    <!-- -   [Docker](#docker)
        -   [GitHub Actions](#github-actions)
            -   [git-cliff-action](#git-cliff-action)
            -   [setup-git-cliff](#setup-git-cliff)
        -   [GitLab CI/CD](#gitlab-cicd)
        -   [Configuration File](#configuration-file) -->
-   [Similar Projects](#similar-projects)
-   [License](#license)
-   [Copyright](#copyright)

</details>

## About

<!-- Scud aims to solve the following problems and integrate them into a single tool for enhancing and accelerating your workflow in any project:

-   Version Control -->
<!-- -   Source Control
-   Code Quality
-   Code Linting -->

### Why build scud?

Scud was originally a tool I wanted to build for myself in an attempt to bridge the gap between making changes to my codebase locally and seeing them reflected on a remote repository. I wanted to be able to do this without having to repeatedly write a series of verbose commands, use an assortment of various tools to get the job done or go through some unique setup process for each new project I created.

## Goals

I was interested in a tool that would provide:

-   **Declarative Version and Source Control** is a set of high-level, declarative operations for accelerating the repetitive tasks common in the development process of any project.
-   **Agnostic to codebase internals and development environment**, not tied to your codebase's language (i.e. works in Rust-based projects, but not JavaScript, Python), or to the developer's shell (i.e. works in Fish, but not Zsh, Bash, Elvish, etc.).
-   **Easy to use**, with a simple, intuitive interface.
-   **Fast**, performance isn't hindered by the underlying implementation details.
-   **Useful diagnostics**, inspired by Rust's compiler diagnostics, general usage information and error messages should be clear and concise with expressive syntactic and semantic highlighting.
    <!-- -   **Hot Swappable**, is simple to mix and match various version control systems and source control providers and allows the developer the ability to migrate between them with ease. Think Prisma but for version control and source control. -->
    <!-- -   **A gentle learning curve** with reasonable familiarity for developers intimate with the underlying components (e.g. git, GitHub CLI, git-cliff, etc.). -->

<!-- TODO auto-generate changelog on commit via git-cliff -->

## Status

Scud is currently a work in progress, so its complete functionality is not yet finished. However, feel free to use it now, and update it when available. Scud will notify you during usage whenever a new update is available and provides a convenient update command, `scud update [alias: scud up]`, which will update your local copy of scud to the latest version available.

<!-- Detailed below is a list of the current features and their status, as well as the roadmap for the future.

-   [x] stage
    -   [x] Git
    -   [ ] Mercurial
    -   [ ] Breezy
    -   [x] Under the hood info
-   [x] unstage
    -   [x] Git
    -   [ ] Mercurial
    -   [ ] Breezy
    -   [ ] Under the hood info
-   [x] commit
    -   [x] Git
    -   [ ] Mercurial
    -   [ ] Breezy
    -   [x] message-formatting
        -   [x] Conventional Commit Standard
        -   [ ] Angular Commit Standard
        -   [ ] None
-   [ ] commit-all
    -   [ ] Git
    -   [ ] Mercurial
    -   [ ] Breezy
-   [x] healthcheck
    -   [x] version control systems
        -   [x] Git
        -   [x] Mercurial
        -   [x] Breezy
    -   [x] source control providers
        -   [x] GitHub
        -   [x] GitLab -->

# Roadmap

-   [ ] Implement Shell tab completions for scud via [`clap_complete`](https://crates.io/crates/clap_complete) crate
-   [ ] Create man page for scud available for users by using `man scud` via [`clap_mangen`](https://crates.io/crates/clap_mangen) crate
    <!-- -   [ ] Add support for [`gitoxide`](https://github.com/Byron/gitoxide) as underlying VCS -->
    <!-- TODO check out smart-release -->
    <!-- -   [ ] Experiment with distributing tool via docker container to allow for even faster onboarding of new developers by not having to install scud's dependencies on the developer's machine -->
-   [ ] In the future, I would like to add support for other version control systems, such as Mercurial and Breezy, as well as source control providers, such as GitHub, GitLab and BitBucket.
<!-- -   [ ] Autogenerated Changelog for git via [`git-cliff`](https://crates.io/crates/git-cliff) or [``], will have to investigate tools for other VCSs -->

_NOTE: I aim to round out the core feature set of scud before exploring further functionality_

<p align="right">(<a href="#top">back to top</a>)</p>

## Features and Functionality

<!-- For scud's declarative version control features (e.g. `commit`, `commit-all`, `push`, `pull`, `stage`, `state`, `unstage`), scud is smart enough to detect the underlying version control system in which it is being used and will automatically use the appropriate commands, so you don't have to worry about the underlying implementation details, it just works. -->

### ⚡ Supercharged Commits

Scud supports a commit workflow that can produce commit messages following an assortment of commit message formats (Angular, Conventional, etc.) enabling developers to create human-readable commit messages in a simple, modular, and easily configurable manner.

### 🛤 Declarative and Rich Version Control Primitives

Scud provides a declarative, concise interface to the underlying version control system, allowing developers to create and iterate on software in a simple yet powerful fashion.

<!-- TODO -->

## Getting Started

You can get started using scud by following one of the installation methods detailed below. Additionally, you must have the required prerequisites installed before you start using scud.

## Prerequisites

To get started using scud, you must have [Rust](https://www.rust-lang.org/) installed on your local machine which can be done via the simple one-liner,

```bash
# Install Rust and toolchain manager, rustup.
curl https://sh.rustup.rs -sSf | sh
```

as seen on the official [Rust website](https://www.rust-lang.org/tools/install).

## Installation

### From crates.io

[scud](crates.io/crates/scud) can be installed from crates.io:

```sh
# Install scud from crates.io.
$ cargo install scud
```

<p align="right">(<a href="#top">back to top</a>)</p>

## Usage

### Command Line Arguments

```
scud [SUBCOMMAND] [OPTIONS]
```

**Options:**

```
    -h, --help       Print help information
    -V, --version    Print version information
```

**Subcommands:**

<!--
# Useful for streamlining the entire process of creating a local repository for your given project/app/library and getting a remote repository
# up and running in a matter of seconds. Supports a variety of different combinations of version control systems and source control providers.
new            Creates a new local repository in the current directory with a specified VCS, if one does not already exist (local repo).
               Additionally, initializes a corresponding remote repository with a specified source control provider. [alias: n] -->

```
# Useful for creating, updating, and deleting branches as well as visualizing and switching between them.
branch        Handles CRUD operations when it comes to repository branching. [alias: br]

# Useful when you have reached a codebase state you want to remember
commit         Commits all staged files in the current local repository. [alias: c]

# Useful for further streamlining the stage and commit process.
commit-all     Stages all modified files in the current local repository and then commits them. [alias: ca]

# Useful for checking to see if your system is setup to work with all of the features of scud.
healthcheck    Checks to see required dependencies are installed. [alias: hc]

# Useful for quickly checking scud's subcommands and options.
help           Print this message or the help of the given subcommand(s).

# Useful for initializing a new local repository.
init           Initializes a local repository with a given VCS provider (currently supported: git, mercurial, breezy). [alias: i]

# Useful for pushing your local commits to the remote repository.
push           Pushes all commits to the remote repository. [alias: ps]

# Useful for pulling remote commits to your local repository.
pull           Pulls all commits from the remote repository. [alias: pl]

# Useful for staging all modified files in your local repository, making them ready to be committed.
stage          Stages all modified files in the current local repository ensuring they are ready to be committed. [alias: s]

# Useful for reverting changes made to files tracked by your version control system.
unstage        Unstages all modified files in the current local repository so they are ready to be committed again. [alias: u]

# Useful for useful updating scud to the latest version.
update         Handles the process of updating scud to the latest version. [alias: up]
```

_NOTE: Many of scud's subcommands (where deemed appropriate) support the `--dry-run` flag for testing usage in a low-stakes environment, as well as the `--info` flag for getting a better idea of the operations scud is performing under the hood when the subcommand is issued._

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- TODO renovate.json -->

## Similar Projects

-   [cocogitto](https://crates.io/crates/cocogitto) - Cocogitto is a set of cli tools for the conventional commit and semver specifications.
-   [Commitizen](https://github.com/commitizen-tools/commitizen) - Create committing rules for projects 🚀 auto bump versions ⬆️ and auto changelog generation 📂
-   [cz cli](https://github.com/commitizen-tools/commitizen) - The commitizen command line utility. #BlackLivesMatter
-   [gitnow](https://github.com/joseluisq/gitnow) - Speed up your Git workflow. 🐠
-   [gitflow](https://github.com/nvie/gitflow) - Git extensions to provide high-level repository operations for Vincent Driessen's branching model.

<!-- -   [git-journal](https://github.com/saschagrunert/git-journal) - The Git Commit Message and Changelog Generation Framework
-   [clog-cli](https://github.com/clog-tool/clog-cli) - Generate beautiful changelogs from your Git commit history
-   [relnotes](https://crates.io/crates/relnotes) - A tool to automatically generate release notes for your project.
-   [cocogitto](https://github.com/oknozor/cocogitto) - A set of CLI tools for the conventional commit
    and semver specifications.
-   [cliff-jumper](https://github.com/favware/cliff-jumper) - A NodeJS CLI tool that combines git-cliff and
    [conventional-recommended-bump](https://github.com/conventional-changelog/conventional-changelog/tree/master/packages/conventional-recommended-bump)
    to semantically bump a NodeJS package and generate a git-cliff powered changelog. -->

<p align="right">(<a href="#top">back to top</a>)</p>

## License

[MIT](https://opensource.org/licenses/MIT)

## Copyright

Copyright © 2022, [Josh Kersey](mailto:pulanski12@gmail.com)

<p align="right">(<a href="#top">back to top</a>)</p>
