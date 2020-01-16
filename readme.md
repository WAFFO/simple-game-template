# Game Template

### Purpose

Game template will be what other games will be built off of. First we want to make what we can, and determine what should be part of the game, and what should be part of the template.

## Build

Requires [rustup](https://www.rust-lang.org/en-US/install.html) and [npm](https://nodejs.org/en/).

Requires [engine](), make sure this is installed on the directory level as this repo.

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
