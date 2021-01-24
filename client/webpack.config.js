const path = require("path");
const glob = require("glob");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CssoWebpackPlugin = require("csso-webpack-plugin").default;
const PurgecssPlugin = require("purgecss-webpack-plugin");

module.exports = {
    entry: {
        click: "./src/pages/click/index.js",
        press: "./src/pages/press/index.js",
        trackpad: "./src/pages/trackpad/index.js",
        slide: "./src/pages/slide/index.js",
        number: "./src/pages/number/index.js",
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
        new PurgecssPlugin({
            paths: glob.sync(`${path.join(__dirname, "public")}/**/*`, { nodir: true }),
            safelist: ["flash-animation", "text-danger"]
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
        new HtmlWebpackPlugin({
            filename: "slide.html",
            template: "./public/slide.html",
            chunks: ["slide"],
        }),
        new HtmlWebpackPlugin({
            filename: "number.html",
            template: "./public/number.html",
            chunks: ["number"],
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
            },
            {
                test: /\.s[ac]ss$/i,
                use: [
                    MiniCssExtractPlugin.loader,
                    "css-loader",
                    "sass-loader"
                ]
            }
        ],
    },
};
