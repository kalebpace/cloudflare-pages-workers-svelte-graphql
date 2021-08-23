var path = require('path')

module.exports = {
  mode: 'development',
  target: 'webworker',
  entry: './src/index.ts',
  output: {
    filename: 'worker.js',
    path: path.join(__dirname, 'dist'),
  },
  devtool: 'cheap-module-source-map',
  resolve: {
    extensions: ['.ts', '.tsx', '.js'],
    alias: {
      // While Apollo Server doesn't use the 'fs' Node.js builtin itself,
      // its dependency - graphql-upload - does leverage it.
      // An intention is for Apollo Server 3.x to no longer directly rely on
      // graphql-upload, so this may be re-visited when that release occurs.
      fs: path.resolve(__dirname, './null.js'),

      // The 'net' and 'tls' Node.js built-in usage within Apollo Server
      // is merely to run `instanceof` checks against an existing,
      // user-supplied "server" instance when subscriptions are desired to
      // be bound to an already-created server.  For the purposes of
      // Cloudflare, where none of these Node.js builtins exist, this
      // instanceof check is irrelevant because such a class could not
      // exist.
      net: path.resolve(__dirname, './null.js'),
      tls: path.resolve(__dirname, './null.js'),
    },
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        loader: 'ts-loader',
        options: {
          // transpileOnly is useful to skip typescript checks occasionally:
          // transpileOnly: true,
        },
      },
    ],
  },
  optimization: {
    usedExports: true,
  },
}
