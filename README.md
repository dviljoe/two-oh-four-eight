# two-oh-four-eight

A walkthrough implementation of 2048 in Rust using Bevy

 - https://www.rustadventure.dev/2048-with-bevy-ecs 
 - https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md
 
## Requirements

Development is done using Nix to manage the installed Rust components. Check usage below for more details and a list of tools to help with this.

## Usage
 
```shellsession
nix shell
cargo run
```

## Using Nix and direnv

For some nice automation, install the following tools and then allow `direnv` to load the nix flake.

- [Nix](https://nixos.org/download)
- [direnv](https://direnv.net/)
- [nix-direnv](https://github.com/nix-community/nix-direnv)
 
```shellsession
direnv allow
```
