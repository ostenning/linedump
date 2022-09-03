# linedump
_Find the file, line, column and function name of an associated address for a given ELF file_

A small terminal application written in Rust for dumping the file, line, column and function name information for an associated address.
Useful for debugging hard fault exceptions when all you have is a program counter register value.

## example:

A hard fault caused by a Cortex-M3 STM32 assembly `udf` instruction yields: 

`cargo run 134621640 ~/file.elf`

```
found: ./asm/inline.rs 181 5
function: _ZN3lib6inline5__udf17ha5cfb7665e0fe90cE, lib::inline::__udf
function: __udf, __udf
```
