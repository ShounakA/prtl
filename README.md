# prtl
Small tool to keep track of your tagged directories. Portal to tagged locations quickly.

## Installation
TBD on where to find the install

## Build from Source
- ```git clone https://github.com/ShounakA/prtl.git```
- ```cd prtl && cargo build --release```

## Usage

- ```prtl --help``` -> Help command
- ```prtl set <path>``` -> Sets the given path (relative or full) as your default prtl
- ```prtl set <path> -t <tag>``` -> Sets the given path to the given tag


- ```prtl get``` -> Gets your default prtl, and prints to stdout
- ```cd $(prtl get)``` -> Take the prtl to your default directory
- ```cd $(prtl get <tag>)``` -> Take the prtl to a tagged prtl


## Contribute
Hello, if you stumble upon this repo and think it is worthy of your time you may contribute in the future.
Currently, I don't have and PR templates, tests, or guides setup. But I may add them soon, stay tuned!  