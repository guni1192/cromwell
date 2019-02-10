# Cromwell

[![CircleCI](https://circleci.com/gh/guni1192/cromwell/tree/master.svg?style=svg)](https://circleci.com/gh/guni1192/cromwell/tree/master)
![crates.io](https://img.shields.io/crates/v/cromwell.svg)
![docs](https://docs.rs/cromwell/badge.svg)
[![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)

Rust Rootless Container Runntime

![logo](./logos/facebook_cover_photo_2.png)


## Dependency

Enable user namespaces

```
$ sudo sysctl kernel.unprivileged_userns_clone=1
```

## Installation

```
$ cargo install cromwell
```

or 

```
$ cargo install --git https://github.com/guni1192/cromwell
```

## Usage

```
cromwell 0.1.1
Takashi IIGUNI <ad2314ce71926@gmail.com>
Rust Rootless Container Runntime

USAGE:
    cromwell [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    run     run cromwell container
```

### Run Container

```bash
$ cromwell run -n <CONTAINER_NAME> --exec /bin/bash
[root@<CONTAINER_NAME> /]# 
```

```bash
$ cromwell run -n <CONTAINER_NAME> --exec 'ls -al'
bin  boot  dev  etc  home  lib  lib64  mnt  opt  proc  root  run  sbin  srv  sys  tmp  usr  var
```

### Pull Image from DockerHub

```
$ cromwell pull -n library/alpine:latest
```

## Test

```
$ cargo test
```

## Build

```
$ cargo make --makefile release.toml workflow
```
