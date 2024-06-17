const path = require("path");
const TerserPlugin = require("terser-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(process.cwd(), "../pkg/dist");

module.exports = {
  mode: "production",
  entry: {
    cjs: path.resolve(process.cwd(), "../pkg/index.cjs.js"),
    esm: path.resolve(process.cwd(), "../pkg/index.esm.js"),
    iife: path.resolve(process.cwd(), "../pkg/index.iife.js")
  },
  output: {
    path: dist,
    filename: "[name].min.js"
  },
  optimization: {
    minimize: true,
    minimizer: [new TerserPlugin()],
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory:  path.resolve(process.cwd(), "../"),
    }),
  ]
};