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
