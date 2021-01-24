const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CssoWebpackPlugin = require("csso-webpack-plugin").default;

module.exports = {
    entry: {
        click: "./src/pages/click/index.js",
        press: "./src/pages/press/index.js",
        trackpad: "./src/pages/trackpad/index.js",
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
        new HtmlWebpackPlugin({
            filename: "press.html",
            template: "./public/press.html",
            chunks: ["press"],
        }),
        new HtmlWebpackPlugin({
            filename: "trackpad.html",
            template: "./public/trackpad.html",
            chunks: ["trackpad"],
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
