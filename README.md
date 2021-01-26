# JackVM Player

This repository contains the source for a Jack language Virtual Machine that runs on the web.  The Jack language is a high-level language you build as part of [nand2tetris](https://www.nand2tetris.org) (aka The Elements of Computing Systems).

Previously, Jack programs only run for those who've installed the nand2tetris software suite on their desktops. This project aims to allow people to share their work more easily on the web.

Rough Demo of JackVM Player:

![Rough Demo](./doc/rough-demo.png)

## Building

This project was built using the rust-webpack template, so it follows that structure. To build and run locally:

```sh
> npm install
> npm run build
```

## Running via the Webpack Dev Server

You can run this project in webpack dev mode, and automatically test changes to any of the JS or rust code used to build this project:

1. Run `npm run start`
2. The browser will automatically load the JackVM Player with a simple program.

## Running tests

```sh
> cargo test
```

## Running other JackVM Programs

The JackVM Player currently only runs Jack language "VM" files (`*.vm`). For now, you can convert any Jack language program into a series of `.vm` files using the nand2tetris software suite:

```sh
> cd projects/11/Pong
> ../../../tools/JackCompiler.sh .
> cat *.vm > pong_complete.vm
```

Then paste the contents of pong_complete.vm into the text field and click "run".

## Author

Jim Connell 2021
