# Project Template Manager
CLI Manager for Project Templates

>Info: You can use only built-in templates until next update. :(

## Build
```sh
git clone https://github.com/Samet195/Project-Template-Manager/
cargo install --path Project-Template-Manager/
```

## Usage
```
Project Template Manager 0.1.0
CLI Manager for Project Templates

USAGE:
    ptm <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    list    Lists avaible templates
    use     Select a template for use
```
The `use` command:
```
ptm-use
Select a template for use

USAGE:
    ptm use [OPTIONS] <TEMPLATE> [PATH]

ARGS:
    <TEMPLATE>    Template ID
    <PATH>        Path to extract

OPTIONS:
    -g, --git     Initialize with git repository
    -h, --help    Print help information
```
The `list` command:
```
ptm-list
Lists avaible templates

USAGE:
    ptm list

OPTIONS:
    -h, --help    Print help information
```
