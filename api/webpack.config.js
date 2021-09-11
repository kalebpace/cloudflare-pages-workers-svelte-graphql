const path = require('path')

module.exports = {
  target: 'webworker',
  entry: './src/index.ts',
  mode: 'development',
  optimization: {
    usedExports: true
  },
  output: {
    filename: 'worker.js',
    path: path.join(__dirname, 'dist')
  },
  devtool: 'cheap-module-source-map',
  resolve: {
    extensions: ['.ts', '.tsx', '.mjs', '.js'],
    alias: {
      fs: path.resolve(__dirname, './null.js'),
      net: path.resolve(__dirname, './null.js'),
      tls: path.resolve(__dirname, './null.js'),
    }
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        loader: 'ts-loader',
        options: {
          // transpileOnly is useful to skip typescript checks occasionally:
          // transpileOnly: true,
        }
      }
    ]
  }
}
