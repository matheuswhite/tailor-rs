# Tailor

Tailor is a C package manager inspired by Rust's Cargo.

## Goals

Tailor aims to make it easy to create, build, clean, run, and manage external dependencies for C projects. These are the guidelines for the project:

- **Don’t reinvent the wheel**: Prefer existing (or compatible) standards instead of inventing new ones. For example: TOML for the package manifest, Git for hosting dependencies, and `compile_commands.json` for IDE tooling.
- **Stay backward-compatible**: Tailor versions are called **editions**. Each new edition adds features while remaining compatible with previous editions.
	- Edition format: `YEAR.SEQUENCE` (for example: `2026.1`, `2026.2`, `2027.30`).
- **Stay focused**: Tailor is a package manager. It’s designed to integrate with other tools, not replace them.
- **Be extensible**: Starting in edition `2026.2`, Tailor will support extending core commands via custom scripts.

## Installation

Currently, Tailor is installed by building from source and placing the executable in a directory on your `PATH`.

Build with Cargo:

```sh
cargo build --release
```

## Package types

Tailor supports two package types:

- `bin`: builds an executable (an application). `bin` packages cannot be used as dependencies.
- `lib`: builds a static library that can be used as a dependency by other packages.

## Package structure and manifest

Tailor lets you choose which source files and include directories are used for compilation.

- **Sources**: by default, all `.c` files under `src/` (equivalent to `src/*.c`) are included.
	- Override this by setting the `src` key in `Tailor.toml` to a list of strings.
	- Each entry may be a glob pattern or a specific file path.
	- Paths are always relative to the `Tailor.toml` location.
- **Include directories**: by default, `include/` is added to the include path.
	- Override this by setting the `include` key in `Tailor.toml` to a list of strings.
	- For library authors, prefer putting headers under a subfolder (for example `include/<libname>/...`) to reduce header name collisions.
- **Defines/options**: you can pass preprocessor defines via `options`. Options can be set for the package itself and/or per dependency.

## Usage

### Create a package

Create a new binary package named `hello` under `resource/`:

```sh
tailor new resource/hello
```

The package name is taken from the last path segment (`hello`). For a binary package, Tailor creates:

- `src/main.c` (a Hello World program)
- `Tailor.toml` (the manifest)

Create a library package instead by passing `--lib`:

```sh
tailor new --lib resource/hello
```

For a library package, Tailor creates:

- `src/hello.c`
- `include/hello/hello.h`
- `Tailor.toml`

### Build a package

From outside the package directory:

```sh
tailor build resource/hello
```

Or from inside `resource/hello`:

```sh
tailor build
```

By default Tailor builds in debug mode and writes build artifacts to `build/debug/`. For release builds, add `--release`:

```sh
tailor build --release resource/hello
```

Debug builds use `-Og`; release builds use `-Os`.

### Run a package

Run follows the same path rules as build:

```sh
tailor run resource/hello
```

Or from inside `resource/hello`:

```sh
tailor run
```

Run in release mode with `--release`:

```sh
tailor run --release resource/hello
```

Running is equivalent to executing the built binary directly:

```sh
build/debug/hello@0.1.0
```

Or in release mode:

```sh
build/release/hello@0.1.0
```

The difference is that `tailor run` builds first if needed. Note: library packages cannot be run.

### Clean a package

From outside the package directory:

```sh
tailor clean resource/hello
```

Or from inside the package:

```sh
tailor clean
```

`clean` removes build output directories and reports how many files were removed and how much disk space was freed.

## Contributing

Feel free to request features or report bugs by opening a GitHub issue. Issues are grouped into milestones, and each milestone is tied to an edition.
