# TFC: Server

[![Crates.io](https://img.shields.io/crates/v/tfc-server)](https://crates.io/crates/tfc-server)
![License](https://img.shields.io/crates/l/tfc-server)

A WebSocket server that uses [TFC](https://crates.io/crates/tfc) for remote
control. The repo contains an iOS app that connects to the server to allow for
remotely controlling a host on the same WiFi network that's running the server.
An Android app is on the roadmap.

## Installation

```shell
cargo install tfc-server
```

## Usage

Run `tfc-server`. Open the app and ensure that both the client and the server
are on the same network. Then enter the host name of the server
(`my-computer.local` for example) into the app.

Currently, the iOS app is not on the App Store. The Xcode project must be
downloaded from the repo and compiled.

## iOS Remote app

See
[docs/screenshots](https://github.com/Kerndog73/The-Fat-Controller/tree/master/docs/screenshots)
for screenshots of the iOS app.
