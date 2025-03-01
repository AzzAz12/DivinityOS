## Introduction
A very very simple microkernel operating system written in rust

## How to build

### Dependency
To build the system, you need to install them on your computer:

- x86_64 rust toolchain (nightly version, at least v1.80)
- xorriso
- limine
- qemu-system-x86_64 to run the os image
### Steps
- `cd` to the project directory and simply execute `./build-image.sh`, the `image.iso` will be generated on the project root

## How to run
- `qemu-system-x86_64 /path/to/image.iso`

## License
MIT License
