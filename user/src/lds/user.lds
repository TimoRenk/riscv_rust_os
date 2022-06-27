OUTPUT_ARCH( "riscv" )

ENTRY( _start )

MEMORY
{
  ram (wxar): ORIGIN = 0x80100000, LENGTH = 128M
}

PHDRS
{
  text PT_LOAD;
  rodata PT_LOAD;
  data PT_LOAD;
  bss PT_LOAD;
}

SECTIONS
{
  . = 0x80100000;
  .text : {
    . = ALIGN(16);
    *(.text.init) *(.text .text.*)
    . = ALIGN(16);
  } >ram AT>ram :text

  .rodata : {
    . = ALIGN(16);
    *(.rodata .rodata.*)
  } >ram AT>ram :rodata

  .data : {
    . = ALIGN(4096);
    *(.sdata .sdata.*) *(.data .data.*)
  } >ram AT>ram :data

  .bss :{
    . = ALIGN(16);
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .);
  } >ram AT>ram :bss

  PROVIDE(_memory_start = ORIGIN(ram));
  PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));
  PROVIDE(_stack_start = _bss_end);
  PROVIDE(_stack_size = 0x80000); /*(524 KiB)*/
  PROVIDE(_stack_end = _stack_start + _stack_size);
}
