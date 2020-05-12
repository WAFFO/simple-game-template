# Game Template

### Purpose

Game template will be what other games will be built off of. First we want to make what we can, and determine what should be part of the game, and what should be part of the template.

## sawd_glm

This project requires you also download my other project: [sawd_glm](https://github.com/WAFFO/sawd-glm/)

Update the `Cargo.toml` file with the path that best suits you.

## Build

Requires [rustup](https://www.rust-lang.org/en-US/install.html) and [npm](https://nodejs.org/en/).

Requires [engine](https://github.com/WAFFO/goblin-engine), make sure this is installed on the directory level as this repo.

IE, your directory should look like this:  
```bash
/Users/you/Workspace/rust/ $ ls
engine          game-template
```

First, build with:

```
$ ./build.sh
```

Then, start server with:

```
$ ./start_server.sh
```

and then visit http://localhost:8080 in your browser! Simple as that!

(If your build asks to update wasm-bindgen-cli, run `update_build.sh` and then build again.)
