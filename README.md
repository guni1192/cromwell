# Cromwell

[![Build Status](https://travis-ci.org/guni1192/ace.svg?branch=master)](https://travis-ci.org/guni1192/ace)

Ownership Managed Container Runntime

## Dependency

- arch-install-scripts
  - pacstrap

## Usage

```bash
$ ace run -n <CONTAINER_NAME>
[root@<CONTAINER_NAME> /]# 
```

```bash
$ ace run -n <CONTAINER_NAME> --exec 'ls -al'
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
