# learn_password

## Description

`learn_password` is a simple terminal tool to check if you have memorize
a new password.

It basically store one of your password hashed ( sha512 ) in a config file
 and compare it in the test/train phase.

## Installation

### Pre-compile bin

You can download the last version <https://github.com/doubleailes/learn_password/releases>

#### Windows

Support Windows 10 x86

#### Linux

Support current linux distro tested Fedora 37, Centos 7, Manjaro 22

#### Darwin

Untested feedbacks welcomes

## Usage

### Help

`learn_password(.exe) --help`

```bash
Usage: learn_password(.exe) [OPTIONS]

Options:
  -t, --train        Start the training
  -s, --store        Store a new password
  -p, --path         Display config path
  -n, --name <NAME>  Config name
  -h, --help         Print help
  -V, --version      Print version
```

### Store

Use  `learn_password(.exe) --store` to store a new password in default config entry.

```bash
$ learn_password -s
Type a password:
```

#### Store specifig config

To store a different password, just name the config.

Use  `learn_password(.exe) --store --name toto`

And it will store the password in a different config file using name argument

### Train

Use `learn_password(.exe) --train` to train at the default config password

```bash
learn_password -t
Type a password: 
...
Type a password: 
Type a password: 
Type a password: 
Type a password: 
You score 9 in a row in 18.95485371s
```

#### Train specifig config

To start training with another config than default.

Use  `learn_password(.exe) --train --name toto`

### Check path

If you wanna check the default config file path of your system

`learn_password(.exe) --path`

```bash
learn_password -p
~/.config/learn_password/default-config.toml
```

#### Check specific config path

To check multiple config, just name the config.

Use  `learn_password(.exe) --path --name toto`

And it will display the config named path
