const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CssoWebpackPlugin = require("csso-webpack-plugin").default;

module.exports = {
    entry: {
        click: "./src/pages/click/index.js",
    },
    output: {
        filename: "js/[name].js",
        publicPath: "/",
        path: path.resolve(__dirname, "dist"),
    },
    plugins: [
        new MiniCssExtractPlugin({
            filename: "css/[name].css"
        }),
        new CssoWebpackPlugin(),
        new HtmlWebpackPlugin({
            filename: "click.html",
            template: "./public/click.html",
            chunks: ["click"],
        }),
    ],
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: [
                    MiniCssExtractPlugin.loader,
                    "css-loader"
                ]
            }
        ],
    },
};
