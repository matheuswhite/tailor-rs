# Tailor

A C language package manager, inspired by Rust's cargo package manager

## Goals

This project is designed to provide an easy way to: create, build, run and
manage external dependencies in C language projects. The following topics will
explain the guidelines that will be followed through out the project development.

- **Don't reinventing the wheel**: We'll try to use (or be compatible to) the market
standards instead of invent a new standard. For example, we'll use cmake to
build the project, the Toml file format as the package manifest, and git as dependency hosting.
- **Be retrocompatible**: New features are realsed only in new version of Tailor. Version in Tailor is called editions e each edition is retrocompatible with the previous one. Editions are compound by the year it was launched, followed by a dot with a sequence number. Examples of valid editions are: `2025.1`, `2025.2`, `2026.30`, and so on.
- **Be compatible with existing CMake projects**: We'll integrate Tailor with the already consolidade CMake based projects, such as: Zephyr-RTOS, espressif-SDK, raspberry pi pico SDK, and so on. Our goal isn't change the struture of an existing project to turn this project compatible with Tailor. Instead, we'll design Tailor to adapt itself to work together with other CMake project structures.
- **Be compatible with Kconfig**: Kconfig is the only consolidade way to create compile-time configuration for C. Even so, there isn't a simple way (outside the CMake build structure) to generate a header file with the configs setted from Kconfig. So, we'll design a standalone tool to parse Kconfig and generate header file with configs. So you won't need to use Tailor to use Kconfig in your project. There is also a plan to create new format to replace Kconfig and to be compatible with it.

## How to install

Nowadays, the only way to install Tailor is to build it from source and copy the executable to some folder present in your `PATH`. To compile Tailor, use `cargo`.

## Package Types

In Tailor, there are 2 package types: `bin` or `lib`. The `bin` package type is for build executable, or applications. It cannot be included as dependency for other packages. Meanwhile, the `lib` package type is for compile static library and to be included as dependency in other packages. Both `lib` packages and `bin` packages. There is also a plan to add a third type: `sdk`. This package type is going to allow custom CMake structure to integrate with Tailor.

### Package Structure

It's possible to choose what source files, and include folders, will be used in the package compilation. For default, all `.c` files inside the `src` folder (or the following pattern `src/*.c`) will be added. You can change what source files will be added for compilation, adding `src` key, at `Tailor.toml`, as a list of string. Each string could be a pattern of source files or a single source file. The path must be always relative to `Tailor.toml`. The same is true for include folders, adding the `include` key at `Tailor.toml`, as a list of string. For default, it's added `include/` folder for compilation. As `include/` is added as default, we ask to library developers use a folder inside the `include/` folder to holds its header files. With that, the chance to have ambiguities for include headers will be reduced.

## How to use

### Creating a package

To create a new binary package for C language, with name `hello`, at `resource` folder, we'll use the following command:

```sh
tailor new resource/hello
```

The package has created with the last path name: `hello`. For binary packages, it's created 2 files: `src/main.c` that prints a hello world message and the Tailor manifest (`Tailor.toml`) with package informations.

If you'd like to create a library package, you need to put `--lib` flag after `new`:

```sh
tailor new --lib resource/hello
```

For library package, is created two files for the library itself: `src/hello.c`, and `include/hello/hello.h`; and the Tailor manifest file: `Tailor.toml`.

### Building the package

To build the project, we'll use the following command, if you are at the same folder of previous command:

```sh
tailor build resource/hello
```

or, if you are inside the `resource/hello`, you can omit the last argument:

```sh
tailor build
```

If the package is a library, then a static library will be produced, instead of an executable.

For default, we'll build in debug mode, and all build files will be created inside the `build/debug` folder. To build in release mode, add `--release` after `build`:

```sh
tailor build --release resource/hello
```

As it's known, we'll use CMake to build the project. So the `CMakeLists.txt` file will be created inside the `build/debug` (or `build/release` if it's in release mode). If any content inside the `Tailor.toml` file changes, so a new `CMakeLists.txt` file will be generated.

### Running the package

To run the project, we'll use the same logic of build:

```sh
tailor run resource/hello
```

or, if you are inside the `resource/hello`:

```sh
tailor run
```

Note that, as build, it's possible to run in release mode, adding `--release` flag:

```sh
tailor run --release resource/hello
```

The run command is the same as to execute the following command in terminal:

```sh
build/debug/hello
```

or in release mode:

```sh
build/release/hello
```

The only difference is it'll build before run the compiled program. Note it's not possible to run library packages.

## How to contribute

Feels free to request features or to report a bug. To do that, create a issue in github. The issues are grouped in milestones. Each milestone is related with an Edition.
