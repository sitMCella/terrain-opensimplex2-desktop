# Terrain Generation using OpenSimplex 2

## Table of contents

* [Introduction](#introduction)
* [Development](#development)
    * [Setup Development](#setup-development)
    * [Build Project Development](#build-project-development)
* [Run Application](#run-application)
    * [Terrain Application](#terrain-application)
    * [Control Panel](#control-panel)

## Introduction

Generation and visualization of a terrain created using OpenSimplex 2 with 3D noise.

This project includes code originally made available under CC0 1.0 Universal (Public Domain).
Original source: https://github.com/KdotJPG/OpenSimplex2

## Development

The project consists of a Rust project for the visualization of a terrain using the winit and the tree-d libraries.

The terrain is generated using the OpenSimplex 2 algorithm embedded as Rust code.

### Setup Development

Install Rust and Cargo. Recommended version:
- Rustc and Cargo >= 1.88.0

### Build Project Development

#### Format Code (Backend)

```sh
cargo fmt
```

#### Build

``` sh
cargo build  --release
```

#### Run application

#### Terrain Application

``` sh
./target/release/terrainopensimplex2
```

#### Control Panel

Open in the browser the file "control_panel.html".
