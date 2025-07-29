# Tailor

A C language package manager, inspired by Rust's cargo package manager

## Goals

This project is designed to provide an easy way to: create, build, run and
manage external dependencies in C language projects. The following guidelines will be followed through out the project development.

- **Do NOT reinvent the wheel**: We'll try to use (or be compatible to) the market
standards instead of inventing a new standard. For example, we'll use cmake to
build the project, the Toml file format as the package manifest, and git as dependency hosting.
- **Be retrocompatible**: New features are to be only released in new editions of Tailor. Each edition must be retrocompatible with the previous one. Editions are composed by the year it was launched, followed by a dot with a sequence number. Examples of valid editions are: `2025.1`, `2025.2`, `2026.30`, and so on.
- **Be compatible with existing CMake projects**: We'll integrate Tailor with already consolidated CMake based projects, such as: Zephyr-RTOS, espressif-SDK, raspberry pi pico SDK, and so on. Our goal isn't to change the struture of an existing project to turn it compatible with Tailor. Instead, we'll design Tailor to adapt itself to work together with other CMake project structures.
- **Be compatible with Kconfig**: Kconfig is the only consolidade way to create compile-time configurations for C. Even so, there isn't a simple way (outside the CMake build structure) to generate a header file with the configs set from Kconfig. So, we'll design a standalone tool to parse Kconfig files and generate header files with said configs. So you won't be forced to use Tailor to manipulate Kconfig in your project. There is also a plan to create a new format to replace Kconfig but also be compatible with it.

## How to install

Currently, the only way to install Tailor is to build it from source and copy the executable to some directory in your `PATH`. To compile Tailor, use `cargo`.

## Package Types

In Tailor, there are 2 package types: `bin` or `lib`. The `bin` package type is for build executable, or applications. It cannot be included as dependency for other packages. Meanwhile, the `lib` package type is to compile it as a static library and be included as dependency in other packages. There is also a plan to add a third type: `sdk`. This package type is going to allow custom CMake structures to integrate with Tailor.

### Package Structure

It's possible to choose what source files, and include folders, will be used in the package compilation. For default, all `.c` files inside the `src` folder (or the following pattern `src/*.c`) will be added. You can change what source files will be added for compilation, adding `src` key, at `Tailor.toml`, as a list of strings. Each string can be a regular expression of source files or a single source file. The path must be always relative to `Tailor.toml`. The same is true for include directories, adding the `include` key at `Tailor.toml`, as a list of strings. For default, the `include/` directory is included for compilation. As `include/` is the default include directory, we ask to library developers use it to hold header files. With that, the chance to have ambiguous headers will be reduced.

## How to use

### Creating a package

To create a new binary package for C language, with the name `hello`, at the `resource` directory, we'll use the following command:

```sh
tailor new resource/hello
```

The package will be created with the last path name, i.e. `hello`. For binary packages, it's created 2 files: `src/main.c` that consists of a hello world binary and the Tailor manifest (`Tailor.toml`) with package informations.

If you'd like to create a library package, you need to put `--lib` flag after `new`:

```sh
tailor new --lib resource/hello
```

For library packages, it is created three files for the library: `src/hello.c`, `include/hello/hello.h` and the Tailor manifest file: `Tailor.toml`.

### Building the package

To build the project, use the following command, if you are at the same directory of the previous command:

```sh
tailor build resource/hello
```

or, if you are inside the `resource/hello`, you can omit the last argument:

```sh
tailor build
```

If the package is a library, then a static library will be produced, instead of an executable.

By default it builds in debug mode, and all build files will be created inside the `build/debug` folder. To build in release mode, add `--release` after `build`:

```sh
tailor build --release resource/hello
```

As it's known, we'll use CMake to build the project. So the `CMakeLists.txt` file will be created inside the `build/debug` (or `build/release` if it's in release mode). If any content inside the `Tailor.toml` file changes, a new `CMakeLists.txt` file will be generated.

### Running the package

To run the project, we'll use the same logic of build:

```sh
tailor run resource/hello
```

or, if you are inside the `resource/hello`:

```sh
tailor run
```

Note that, just like in the build command, it's possible to run in release mode, adding `--release` flag:

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

The only difference is that it'll build before running the compiled program. Note it's not possible to run library packages.

## How to contribute

Feels free to request features or to report a bug. To do that, create a issue in github. The issues are grouped in milestones. Each milestone is related with an Edition.
