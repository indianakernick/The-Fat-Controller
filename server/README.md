# TFC: Server

A WebSocket server (soon to be a TCP server) that uses
[TFC](https://crates.io/crates/tfc) for remote control. The repo contains an iOS
app that connects to the server to allow for remotely controlling a host on the
same WiFi network that's running the server. An Android app is on the roadmap.

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

See [docs/screenshots](https://github.com/Kerndog73/The-Fat-Controller/tree/master/docs/screenshots) for more.

<p align="center">
  <img alt="Number pad screenshot" width="300" src="https://github.com/Kerndog73/The-Fat-Controller/raw/master/docs/screenshots/2021-02-04_0.png"/>
  <img alt="Video remote screenshot" width="300" src="https://github.com/Kerndog73/The-Fat-Controller/raw/master/docs/screenshots/2021-02-05_1.png"/>
</p>

<p align="center">
  <img alt="Landscape orientation number pad screenshot" height="300" src="https://github.com/Kerndog73/The-Fat-Controller/raw/master/docs/screenshots/2021-02-04_5.png"/>
</p>
