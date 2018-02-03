require('babel-core/register');
import path from 'path';
import HTMLWebpackPlugin from 'html-webpack-plugin';

const src = path.resolve(__dirname, "src");
const dist = path.resolve(__dirname, "dist");

module.exports = {
  entry: src + "/reactsample.jsx",
  output: {
    path: dist,
    filename: "reactsample.bundle.js"
  },
  
  module: {
    loaders: [
      {
        test: /\.jsx$/,
        exclude: /node_modules/,
        loader: "babel-loader"
      }
    ]
  },

//  resolve: {
//    extensions: ["", ".js"]
//  },

  plugins: [
    new HTMLWebpackPlugin({
      template: src + '/index.html',
      filename: 'index.html'
    })
  ]
}
