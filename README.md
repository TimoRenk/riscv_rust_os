# Risc-V Rust OS

Os written in Rust

## Install

- rustup target install riscv64gc-unknown-none-elf

### GDB

gdb-multiarch/ riscv-elf-gdb

#### Windows

msys2: -> pacman -S mingw-w64-x86_64-toolchain

### objcopy

cargo install cargo-binutils
rustup component add llvm-tools-preview

## QEMU

[Memory Layout](https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c)

``` C
    [VIRT_DEBUG] =        {        0x0,         0x100 },
    [VIRT_MROM] =         {     0x1000,        0xf000 },
    [VIRT_TEST] =         {   0x100000,        0x1000 },
    [VIRT_RTC] =          {   0x101000,        0x1000 },
    [VIRT_CLINT] =        {  0x2000000,       0x10000 },
    [VIRT_ACLINT_SSWI] =  {  0x2F00000,        0x4000 },
    [VIRT_PCIE_PIO] =     {  0x3000000,       0x10000 },
    [VIRT_PLATFORM_BUS] = {  0x4000000,     0x2000000 },
    [VIRT_PLIC] =         {  0xc000000, VIRT_PLIC_SIZE(VIRT_CPUS_MAX * 2) },
    [VIRT_APLIC_M] =      {  0xc000000, APLIC_SIZE(VIRT_CPUS_MAX) },
    [VIRT_APLIC_S] =      {  0xd000000, APLIC_SIZE(VIRT_CPUS_MAX) },
    [VIRT_UART0] =        { 0x10000000,         0x100 },
    [VIRT_VIRTIO] =       { 0x10001000,        0x1000 },
    [VIRT_FW_CFG] =       { 0x10100000,          0x18 },
    [VIRT_FLASH] =        { 0x20000000,     0x4000000 },
    [VIRT_IMSIC_M] =      { 0x24000000, VIRT_IMSIC_MAX_SIZE },
    [VIRT_IMSIC_S] =      { 0x28000000, VIRT_IMSIC_MAX_SIZE },
    [VIRT_PCIE_ECAM] =    { 0x30000000,    0x10000000 },
    [VIRT_PCIE_MMIO] =    { 0x40000000,    0x40000000 },
    [VIRT_DRAM] =         { 0x80000000,           0x0 },
```

## RISC-V

>[!info] [RISC-V Manual](<https://github.com/riscv/riscv-isa-manual/#readme>)
>It contains:
>
>- mcause
>- mstatus
>- mepc

[Register](https://en.wikichip.org/wiki/risc-v/registers)
<https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc#system-reset-extension-eid-0x53525354-srst>
<https://github.com/rust-embedded/riscv>

### mcause

| Interrupt | Exception Code | Description                    |
| --------- |:-------------- |:------------------------------ |
| 1         | 0              | _Reserved_                     |
| 1         | 1              | Supervisor software interrupt  |
| 1         | 2              | _Reserved_                     |
| 1         | 3              | Machine software interrupt     |
| 1         | 4              | _Reserved_                     |
| 1         | 5              | Supervisor timer interrupt     |
| 1         | 6              | _Reserved_                     |
| 1         | 7              | Machine timer interrupt        |
| 1         | 8              | _Reserved_                     |
| 1         | 9              | Supervisor external interrupt  |
| 1         | 10             | _Reserved_                     |
| 1         | 11             | Machine external interrupt     |
| 1         | 12-15          | _Reserved_                     |
| 1         | ≥16            | _Designated for platform use_  |
| 0         | 0              | Instruction address misaligned |
| 0         | 1              | Instruction access fault       |
| 0         | 2              | Illegal instruction            |
| 0         | 3              | Breakpoint                     |
| 0         | 4              | Load address misaligned        |
| 0         | 5              | Load access fault              |
| 0         | 6              | Store/AMO address misaligned   |
| 0         | 7              | Store/AMO access fault         |
| 0         | 8              | Environment call from U-mode   |
| 0         | 9              | Environment call from S-mode   |
| 0         | 10             | _Reserved_                     |
| 0         | 11             | Environment call from M-mode   |
| 0         | 12             | Instruction page fault         |
| 0         | 13             | Load page fault                |
| 0         | 14             | _Reserved_                     |
| 0         | 15             | Store/AMO page fault           |
| 0         | 16-23          | _Reserved_                     |
| 0         | 24-31          | _Designated for custom use_    |
| 0         | 32-47          | _Reserved_                     |
| 0         | 48-63          | _Designated for custom use_    |
| 0         | ≥64            | _Reserved_                     |

## UART

<https://osblog.stephenmarz.com/ch0.html>
<https://os.phil-opp.com/>
<https://github.com/sgmarz/osblog/blob/master/risc_v/src/lds/virt.lds>
<https://github.com/skyzh/core-os-riscv/blob/master/kernel/src/uart.rs>
<https://docs.rust-embedded.org/book/start/qemu.html>

UART
<https://www.lammertbies.nl/comm/info/serial-uart>

Check riscv reader for paper info for register infos in first two lectures

## Plic

<https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc>

## Questions

- Why use mret in setup
- How to avoid race-conditions in UART/ Kernel

### Answered

- Why align to 16?
  - `ALIGN(4096) tells the linker to align the current memory location (which is
       0x8000_0000 + text section + rodata section) to 4096 bytes. This is because our paging
       system's resolution is 4,096 bytes or 4 KiB.`
- >ram AT>ram?
- sdata .sbss
- use wfi?
  - Wait for interrupts

## GDB

- info registers

<https://stackoverflow.com/questions/2420813/using-gdb-to-single-step-assembly-code-outside-specified-executable-causes-error>

- gdbtui. Or run gdb with the -tui switch. Or press C-x C-a after entering gdb.
- layout asm
- Press C-x s
- use si ni
- use gdb-multiarch!
- x/100x $sp
- -exec p/x $mepc

readelf -a user_1 | less

## LLDB

Don't use it!
<https://lldb.llvm.org/use/map.html>

## Tools

### NM

Check memory layout
```x86_64-w64-mingw32-gcc-nm riscv_rust_os | sort```
