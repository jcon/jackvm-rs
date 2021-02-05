# demo

A demo project showing how to integrate the JackVM player on a website. It was built with [create-wasm-app](https://github.com/rustwasm/create-wasm-app). To run locally ensure you have `rustup` and `npm` installed as discussed in the [main README](../README.md).

To run the build and run the demo locally, follow these instructions:

```sh
# Ensure your current directory is this directory.
> cd demo
> npm install
> npm run start
```

## Changes to create-wasm-app

This demo took the create-wasm-app template and did the following:
- Upgraded webpack to 5.x
- Updated package.json to reference '../web/pkg` (the location where the package is built locally)
- Updated the main index file to wait until the JackVmPlayer was loaded asynchronously.


