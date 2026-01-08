# Test Plan

## Instructions

For each test, create a setup and teardown. On teardown, all created files must be deleted. You need to test the program as a blackbox, using the same commands and actions that a user would use. If the program create some file, you need to check the exact string, if the program calls another program, then these files doesn't need comparations.

## Test Cases

### New Package

- Create a new binary package, using this command: `tailor new hello`, and check if the contents are ok;
- Create a new binary package, using the flag `--bin`, and check if the contents are ok;
- Create a new binary package, with a name of an existing folder;
- Create a new library package, and check if the contents are ok;
- Create a new library package, with a name of an existing folder.

### Build Package

### Run Package

### Dependencies
