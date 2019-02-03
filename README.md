# Cromwell

[![CircleCI](https://circleci.com/gh/guni1192/cromwell/tree/master.svg?style=svg)](https://circleci.com/gh/guni1192/cromwell/tree/master)

Rust Rootless Container Runntime

## Dependency

## Usage

```
Cromwell v1.0.0
Takashi IIGUNI <ad2314ce71926@gmail.com>
Ownership Managed Container Runntime

USAGE:
    cromwell [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    run     run cromwell container
```

```bash
$ cromwell run -n <CONTAINER_NAME> --exec /bin/bash
[root@<CONTAINER_NAME> /]# 
```

```bash
$ cromwell run -n <CONTAINER_NAME> --exec 'ls -al'
bin  boot  dev  etc  home  lib  lib64  mnt  opt  proc  root  run  sbin  srv  sys  tmp  usr  var
```
## Test

```
$ cargo test
```

Testing about network module is needed root.  
Their unit test is determined ignore elements because travis ci can not use cargo test in root.  
If you want to test them in local environment, use below command.  

```bash
$ sudo cargo test -- --ignored
```


## Build

```
$ sudo cargo make --makefile release.toml deploy
```
