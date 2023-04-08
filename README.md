# prtl
Small tool to keep track of your tagged directories. Portal to tagged locations quickly.

## Installation
- ```cargo install prtl```

## Build from Source
- ```git clone https://github.com/ShounakA/prtl.git```
- ```cd prtl && cargo build --release```

## Auto Configure Shorthand
```prtl ez-init --shell bash```
## Manually Configure Shorthand

### Bash
1. Add a new shell file `path/to/your/newly/created/script/file.sh` with: 
   ```bash
   function p() {
      if [[ $1 = "get" ]]; then 
         cd $(prtl "$@")
      elif [[ $1 = "set" ]]; then
         $(prtl $@)
      else
         echo Global options will not work. Type \'prtl -h\' for more info.
         echo \'p\' short-hand only supports \'get\' and \'set\' commands. 
      fi
   }
   ```
2. Update your .bashrc | .bash_profile | .profile to include:
``` source path/to/your/newly/created/script/file.sh ```

## Usage

- ```prtl -h``` -> Help command
- ```prtl set <path>``` -> Sets the given path (relative or full) as your default prtl
- ```prtl set <path> -t <tag>``` -> Sets the given path to the given tag


- ```prtl get``` -> Gets your default prtl, and prints to stdout
- ```cd $(prtl get)``` -> Take the prtl to your default directory
- ```cd $(prtl get <tag>)``` -> Take the prtl to a tagged prtl

__With shorthand configured__ 
 - ```p get``` is short for ```cd $(prtl get)```
 - ```p get <tag>``` is short for ```cd $(prtl get <tag>)```
 - ```p set <path>``` is short for ```prtl set <path>```
 - ```p set <path> -t <tag>``` is short for ```prtl set <path> -t <tag>```

## Contribute
Hello, if you stumble upon this repo and think it is worthy of your time you may contribute in the future.
Currently, I don't have any PR templates, tests, or guides setup. But I may add them soon, stay tuned!  