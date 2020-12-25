const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require('path');
const glob = require('glob');

module.exports = {
  entry: {
    bootstrap: path.resolve(__dirname, "./bootstrap.js"),
    style: path.resolve(__dirname, './static/style.css'),
    images: glob.sync(path.resolve(__dirname, './static/images/*.*')),
  },
  /*output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },*/
  output: {
    path: path.resolve('./dist/'),
    publicPath: '/static/',
    filename: '[name].js',
  },
  module: {
    rules: [
      {
        test: /\.css$/,        
        use: ["style-loader", "css-loader"]
      },
      {
        test: /(\.woff2?|\.woff|\.ttf|\.eot|\.svg)(\?v=\d+\.\d+\.\d+)?$/,
        loader: 'file-loader?name=[name]-[hash:6].[ext]',
      },
      {
        test: /\.(png|jpe?g|gif|ico)$/,
        loader: 'file-loader?name=[name].[ext]',
      },
    ],
  },
  mode: "development",
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, ".", "./static/index.html")
    })
  ]
  /*plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],*/
};
