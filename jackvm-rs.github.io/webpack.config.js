const path = require('path');

const srcDir = path.resolve(__dirname, '_js');
const distDir = path.resolve(__dirname, 'js');

module.exports = {
  entry: path.resolve(srcDir, "bootstrap.js"),
  output: {
    path: distDir,
    filename: "bootstrap.js",
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
  },
  devtool: "source-map",
  plugins: [],
};
