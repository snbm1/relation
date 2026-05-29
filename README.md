# Relation
<img src="img/logo.png">

**Relation** is a tiny proxy client based on [sing-box](https://github.com/SagerNet/sing-box).

It provides a simple way to add, manage and run proxy configurations from the terminal, with support for daemon mode, TUN mode, system proxy control and a terminal UI.

## Overview

Relation is designed as a lightweight local proxy manager.

Instead of manually editing sing-box configuration files, you can add a proxy link, save it as a named configuration, run it when needed, and manage it from a simple CLI.

The project combines:

- a Rust-based command-line interface;
- a background daemon for running proxy sessions;
- sing-box as the proxy core;
- optional TUI for interactive usage.

## Features

- Add proxy configurations from URL
- Manage multiple saved configurations
- Run configurations by name or index
- Start and stop proxy sessions from CLI
- Background daemon mode
- TUN / VPN-style mode
- System proxy enable / disable support
- DNS configuration
- Route rules configuration
- Interactive terminal UI
- Lightweight and minimal setup

## Use Cases

Relation can be useful if you want to:

- quickly switch between different proxy profiles;
- run sing-box without manually writing full JSON configs;
- manage proxy configs directly from the terminal;
- use a small local proxy daemon;
- experiment with DNS, routing and TUN-based proxy setups;
- keep several proxy profiles in one place.

## Installation

### Build from source

Requirements:

- Rust
- Go
- Make
- C build tools

Clone the repository:

```bash
git clone https://github.com/snbm1/relation.git
cd relation
```

Build the project:

```bash
make build
```

For an optimized release build:

```bash
make release
```

Install system-wide:

```bash
sudo make install
```

By default, files are installed under `/usr/local`.

## Usage

### Add a proxy configuration

```bash
relation add --url "<proxy-url>"
```

Add a configuration with a custom name:

```bash
relation add --url "<proxy-url>" --name my-proxy
```

Overwrite an existing configuration:

```bash
relation add --url "<proxy-url>" --name my-proxy --rewrite
```

Add a configuration in TUN mode:

```bash
relation add --url "<proxy-url>" --tun
```

## List configurations

```bash
relation list
```

Example:

```text
[ 1]: my-proxy
[ 2]: backup-proxy
```

## Run a configuration

Run by name:

```bash
relation run my-proxy
```

Run by index:

```bash
relation run 1
```

Run without enabling system proxy:

```bash
relation run my-proxy --unable-system-proxy
```

Run quietly in daemon mode:

```bash
relation run my-proxy --quiet
```

## Stop proxy

```bash
relation stop
```

## Stop proxy and shutdown daemon

```bash
relation quit
```

## Check status

```bash
relation status
```

Example output:

```text
Running config: my-proxy
System proxy: true
```

## Manage configurations

Rename a configuration:

```bash
relation manage my-proxy --name new-name
```

Print configuration sections:

```bash
relation manage my-proxy --print
```

Add DNS servers:

```bash
relation manage my-proxy --dns 1.1.1.1 --dns 8.8.8.8
```

Add route rules:

```bash
relation manage my-proxy --route "<route-rule>"
```

## Remove configurations

Remove by name:

```bash
relation remove my-proxy
```

Remove by index:

```bash
relation remove 1
```

Remove all configurations:

```bash
relation remove
```

## Terminal UI

Relation also includes an optional terminal UI:

```bash
relation tui
```

The TUI provides an interactive way to work with saved configurations.

## Project Structure

```text
.
├── go_methods/      # sing-box integration
├── macros/          # Rust procedural macros
├── src/             # main Rust source code
├── Cargo.toml       # Rust package configuration
├── go.mod           # Go module configuration
├── makefile         # build and install commands
└── README.md
```



## Tech Stack

- Rust
- Go
- sing-box
- clap
- tokio
- ratatui
- crossterm

