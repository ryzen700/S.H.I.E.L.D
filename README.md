<div align="center">

[<img src="https://raw.githubusercontent.com/hydralauncher/hydra/refs/heads/main/resources/icon.png" width="144"/>](https://github.com/ryzen700/S.H.I.E.L.D)

  <h1 align="center">S.H.I.E.L.D Launcher</h1>

  <p align="center">
    <strong>S.H.I.E.L.D Launcher is an open-source gaming platform created to manage your gaming library. This project is a fork of <a href="https://github.com/hydralauncher/hydra">Hydra Launcher</a>, written in Node.js (Electron, React, TypeScript) and Rust, with libtorrent providing the torrent engine.</strong>
  </p>

[![build](https://img.shields.io/github/actions/workflow/status/ryzen700/S.H.I.E.L.D/build.yml)](https://github.com/ryzen700/S.H.I.E.L.D/actions)
[![release](https://img.shields.io/github/package-json/v/ryzen700/S.H.I.E.L.D)](https://github.com/ryzen700/S.H.I.E.L.D/releases)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

![S.H.I.E.L.D Launcher Home Page](./docs/screenshot.png)

</div>

## Features

- Add games that you own to your library
- Profile that shows what you are playing to your friends
- Save your game progress in the cloud with cloud save support
- Unlock achievements
- Navigate through a rich catalogue with suggestion algorithms
- Discover new games

## About this Fork

S.H.I.E.L.D Launcher is forked from [Hydra Launcher](https://github.com/hydralauncher/hydra) by Los Broxas. The project is distributed under the terms of the MIT License, preserving original copyrights while serving as the baseline for further development.

## Build from source and contributing

### Local development requirements

- Node.js + Yarn
- Rust toolchain (for `hydra-native`)
- Git and a C++ toolchain (Visual Studio C++ Build Tools on Windows, GCC/Clang on Linux, Xcode command-line tools on macOS). The native build obtains CMake and CTest automatically through vcpkg.

After installing dependencies, `postinstall` builds the Rust native addon automatically (`hydra-native/hydra-native.node`).

The native build includes a Rust wrapper around pinned libtorrent. Development and packaged torrenting do not require Python.

On Windows x64 and Linux x64, the libtorrent bridge comes prebuilt from `native/torrent-bridge/prebuilt`, so installs skip compiling libtorrent through vcpkg. Other platforms build it from source. When a branch push changes `native/torrent-bridge` or `scripts/build-torrent-bridge.cjs`, the "Torrent bridge prebuilts" workflow rebuilds the binaries and commits them to that branch; until then, installs build the bridge from source. Set `HYDRA_TORRENT_BRIDGE_FROM_SOURCE=1` to always build it from source.

## Contributors

<a href="https://github.com/ryzen700/S.H.I.E.L.D/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=ryzen700/S.H.I.E.L.D" />
</a>

Original Hydra Launcher contributors: [hydralauncher/hydra contributors](https://github.com/hydralauncher/hydra/graphs/contributors).

## License

S.H.I.E.L.D Launcher is licensed under the [MIT License](LICENSE).  
Original work Copyright (c) 2024 Los Broxas.  
Modifications Copyright (c) 2026 Damir Zaripov.
