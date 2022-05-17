# Risc-V Rust OS

Os written in Rust

## RISC-V

[Register]<https://en.wikichip.org/wiki/risc-v/registers>
<https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc#system-reset-extension-eid-0x53525354-srst>
<https://github.com/riscv/riscv-isa-manual/#readme>
<https://github.com/rust-embedded/riscv>
[Register]

## UART

<https://osblog.stephenmarz.com/ch0.html>
<https://github.com/sgmarz/osblog/blob/master/risc_v/src/lds/virt.lds>
<https://github.com/skyzh/core-os-riscv/blob/master/kernel/src/uart.rs>
<https://docs.rust-embedded.org/book/start/qemu.html>

UART
<https://www.lammertbies.nl/comm/info/serial-uart>

Check riscv reader for paper info for register infos in first two lectures

## Questions

- How to avoid race-conditions in UART/ Kernel?
- What is mtval?

### Answered

- Why align to 16?
  - `ALIGN(4096) tells the linker to align the current memory location (which is
       0x8000_0000 + text section + rodata section) to 4096 bytes. This is because our paging
       system's resolution is 4,096 bytes or 4 KiB.`
- Align behind text data?
- >ram AT>ram?
- sdata .sbss
- compressed instructions?
- use wfi?

## GDB

- info registers

<https://stackoverflow.com/questions/2420813/using-gdb-to-single-step-assembly-code-outside-specified-executable-causes-error>

- gdbtui. Or run gdb with the -tui switch. Or press C-x C-a after entering gdb.
- layout asm
- Press C-x s
- use si ni
- use gdb-multiarch!
- x/100x $sp

readelf -a user_1 | less

## LLDB

Don't use it!
<https://lldb.llvm.org/use/map.html>

## Tools

### NM

Check memory layout
```x86_64-w64-mingw32-gcc-nm riscv_rust_os | sort```
