# EaseXX
Cross platform build tool for C++ programs. Avoid riddle makefiles or cmakelists.
---

This is a pretty simple tool for building C++ and/or running tests. It is not flexible
as a makefile, but avoid needing to configure everything in order to get a toy program running.

## Config file
```json
./build.json
{
    "outputFileName": "my-program",
    "buildDir": "build",
    "compiler": "clang++",
    "compilerFlags": ["-std=c++20", "-Wall", "-Wextra"],
    "includeDirs": ["include"],
    "libDirs": [],
    "libs": [],
    "devLibDirs": [],
    "devLibs": []
}
```

**NOTE:** `devLibDirs` and `devLibs` fields can only be used in files from `tests/` directory, since
they are only being linked when running the test command.

## Usage
After compiling it and setting up the `build.json` file correctly, you can use the available commands:

### Build
`./easexx build` traverses the  `src/` directory and compile every `.cpp` file found, generating an executable
with the name set on `outputFileName` from `build.json` file.

### Test
`./easexx test` compiles every file from `src/` and `tests/` directory, then compiles and executes each
`_test.cpp`-suffixed file from tests directory as a separated executable.
