const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const ServiceWorkerWebpackPlugin = require('serviceworker-webpack-plugin');

module.exports = {
  mode: 'development',
  entry: './src/choice.tsx',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: [
          {loader: 'ts-loader'},
          {
            loader: 'tslint-loader',
            options: {
              configuration: {
                defaultSeverity: 'warn',
                extends: ['tslint:recommended', 'tslint-react'],
                jsRules: {},
                rules: {
                  quotemark: [true, 'jsx-double', 'single', 'avoid-escape'],
                  'arrow-parens': false,
                  'object-literal-sort-keys': false,
                  'interface-name': false,
                  'trailing-comma': {
                    multiline: ['objects', 'arrays', 'typeLiterals'],
                  },
                },
                rulesDirectory: [],
              },
            },
          },
        ],
        exclude: /node_modules/,
      },
      {test: /manifest\.webmanifest$/, use: 'file-loader'},
      {
        test: /\.less$/,
        use: [{loader: 'style-loader'}, {loader: 'css-loader'}, {loader: 'less-loader'}],
      },
    ],
  },
  devtool: 'source-map',
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  output: {
    filename: 'choice.js',
    path: path.resolve(__dirname, 'dist'),
  },
  devServer: {
    contentBase: path.join(__dirname, 'dist'),
    compress: true,
    port: 9000,
  },
};
