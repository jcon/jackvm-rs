const CopyWebpackPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require('path');

const crateDir = path.resolve(__dirname, '../web');
const distDir = path.resolve(__dirname, 'dist');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: distDir,
    filename: "bootstrap.js",
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
  },
  devtool: "source-map",
  module: {
    rules: [
      // This is a work-around for wasm-pack generated modules. Their package.json files currently
      // are written with { sideEffects: false }. This value doesn't seem to break webpack 3.x, but
      // does break webpack 4+. These modules do actually have side-effects though: they load files
      // external to the package's main module.
      {
        test: () => true,
        sideEffects: true,
      }
    ]
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        {from: 'static', to: distDir },
      ]
    }),
    new WasmPackPlugin({
      crateDirectory: crateDir,
      outName: 'web',
    //   forceMode: 'production',
    })
  ],
};
