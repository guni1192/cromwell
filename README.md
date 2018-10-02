# ACE

[![Build Status](https://travis-ci.org/guni1192/ace.svg?branch=master)](https://travis-ci.org/guni1192/ace)

ArchLinux Container Engine

## Dependency

- arch-install-script
  - pacstrap

## Usage

```bash
$ ace --name <CONTAINER_NAME>
[root@<CONTAINER_NAME> /]# 
```

```bash
$ ace --name <CONTAINER_NAME> --exec 'ls -a'
bin  boot  dev  etc  home  lib  lib64  mnt  opt  proc  root  run  sbin  srv  sys  tmp  usr  var
```
