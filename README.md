# RISC-V Rust OS

A rudimentary operating system for RISC-V written in Rust.
The initial goal for this project was the learning and understanding of OS fundamentals, such as machine versus user level, system calls, context switching, exceptions, interrupts, registers, memory management, and hardware handling.

## Requirements

The project has a VS Code launch configuration. It utilizes the QEMU emulator to deploy the OS and gdb-multiarch for debugging, albeit none is required.

### Target Architecture

Use rustup to install the correct target architecture to build for:

```shell
rustup target install riscv64gc-unknown-none-elf
```

### QEMU

Install [QEMU](https://www.qemu.org/).
To use the launch configuration `qemu-system-riscv64` has to be in `PATH`.

### GDB

To use the launch configuration `gdb-multiarch` has to be in `PATH`.

Alternatively, it is preferable to use `riscv-elf-gdb` if it is available, however, this requires adjusting the launch configuration.

#### Windows

On Windows GDB can be installed using msys2:

```shell
pacman -S mingw-w64-x86_64-toolchain
```
