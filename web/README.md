## jackvm-player

A package that runs compiled Jack programs (.vm files) on the web. Jack is a high-level language you build in "Nand 2 Tetris", (aka "The Elements of Computing Systems"). This VM supports the basic features of the Jack VM included in the nand2tetris suite: the Jack "Operating System", keyboard input, and screen output.

JackVm Player requires a browser capable of running WebAssembly (WASM). Generally any modern browser that's newer than 2017 supports WASM. Check out capatability [on MDN](https://developer.mozilla.org/en-US/docs/WebAssembly#browser_compatibility).

## Basic Usage

**player.js**
```javascript
import { JackVmPlayer } from "jackvm-player";

const parentEl = document.getElementById('screen-container');
let player = new JackVmPlayer(parentEl);

fetch("<concatenated_vm_file_url>")
    .then(res => res.text())
    .then(program => {
        player.load(program);
    });
```

### Webpack Notes

If you're using webpack 4+, you'll need to either load your module that configures the player asynchronously, or enable an experimental flag on Webpack. If you don't you'll see an error that looks like this:

```
WebAssembly module is included in initial chunk. This is not allowed, because WebAssembly download and compilation must happen asynchronous. Add an async splitpoint (i. e. import()) somewhere between your entrypoint and the WebAssembly module:
```

#### Load module asynchronously

**bootstrap.js**
```javascript
import('./player.js')
  .catch(console.error);
```

#### Enaable async webassembly files:

**webpack.config.js**
```javascript
module.exports = {
  // ...
  experiments: {
    asyncWebAssembly: true,
  },
  // ...
}
```

## Creating VM Files

`jackvm-player` currently is able to load a single `.vm` file. It is capable of loading a VM file that declares the entire program though. To create your own .vm file follow these steps:

```shell
# Change your directory to your Jack program source (containing `*.jack` files)
shell$ cd my_jack_program

# Compile the program using the Jack compiler.
shell$ JackCompiler.sh .

# Concatenate all .vm files. The OS files are not needed
shell$ cat *.vm > my_jack_program.vm
```

You can then include `my_jack_program.vm` in your project and load it into `jackvm-player`.