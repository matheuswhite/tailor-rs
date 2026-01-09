# Test Plan

## Instructions

For each test, create a setup and teardown. On teardown, all created files must be deleted. You need to test the program as a blackbox, using the same commands and actions that a user would use. If the program create some file, you need to check the exact string, if the program calls another program, then these files doesn't need comparations.

## Test Cases

### New Package

- Create a new binary package, with no flags, and check if the contents are ok;
- Create a new binary package, using the flag `--bin`, and check if the contents are ok;
- Create a new binary package, with a name of an existing folder;
- Create a new library package, and check if the contents are ok;
- Create a new library package, with a name of an existing folder.

### Build Package

- Run the build command on a binary package, and check if resultant binary has the correct output, shown at `main.c` package file. Also check if the `CMakeLists.txt` has got the correct content;
- Run the build command on a binary package, with the `--debug` flag, and check if resultant binary has the correct output, shown at `main.c` package file. Also check if the `CMakeLists.txt` has got the correct content;
- Run the build command on a binary package, in release mode using the `--release` flag, and check if resultant binary has the correct output, shown at `main.c` package file. In this case, check if the `CMakeLists.txt` created has got the release mode flag and if the content is right;
- Run the build command on a library package, and check if result is a static library. Also check if the `CMakeLists.txt` has got the correct content;
- Run the build command on a library package, with the `--debug` flag, and check if result is a static library. Also check if the `CMakeLists.txt` has got the correct content;
- Run the build command on a library package, in release mode using the `--release` flag, and check if result is a static library. In this case, check if the `CMakeLists.txt` created has got the release mode flag and if the content is right;

### Run Package

### Dependencies
