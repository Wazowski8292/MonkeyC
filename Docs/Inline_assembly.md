# Inline Assembly

An **inline assembly block** lets you write raw x86-64 assembly directly inside a Monkey C function. Like a `struct`, it is a special block with its own keyword, but instead of grouping fields and methods it holds instructions that are placed into the program exactly as you wrote them.

```text
fn main() {
    asm -> 16 {
        mov edi, 42
        call print_int
    }
}
```

This program reserves 16 bytes, loads `42` into `edi` and calls the standard library's `print_int`, which prints `42`.

## Defining an Inline Assembly Block

A block is declared with the `asm` keyword, an arrow with the number of bytes to allocate, and a body in braces. The body holds assembly code.

| Part | Description |
| --- | --- |
| `asm -> size { ... }` | Declares an inline assembly block that reserves `size` bytes. |
| `size` | The amount of bytes to allocate. It is rounded to the closest power of two. |
| `{ ... }` | The assembly code, one instruction per line. |

## Allocation Size

The number after `->` is how many bytes the block reserves. The compiler does not use that exact number: it rounds it to the **closest power of two**. A value exactly halfway between two powers of two rounds up.

| You write | Bytes reserved |
| --- | --- |
| `asm -> 8` | `8` |
| `asm -> 20` | `16` |
| `asm -> 24` | `32` |
| `asm -> 100` | `128` |
| `asm -> 1000` | `1024` |

Because of the rounding, you can treat the size as a minimum you ask for and plan around the number it rounds to, not the one you typed.

## Writing the Assembly

The body is ordinary assembly, so everything in it follows the same rules as the code in the [standard library](./print.md): the first integer argument goes in `rdi`, the first floating point argument goes in `xmm0`, and a call to a library function is a plain `call`.

```text
fn main() {
    asm -> 32 {
        mov edi, 7
        call print_int
        mov edi, 8
        call print_int
    }
}
```

This prints `7` and then `8`.

The block can appear anywhere a statement can, including inside a method of a [struct](./structs.md).

## Things to Keep in Mind

The compiler does not read or check the assembly in a block. A mistake in it is not a Monkey C error: it is an assembler error, or a program that crashes when it runs.

- **Registers.** The compiler's own code uses registers too. Anything you change that the surrounding code relies on, such as `rbp` or `rsp`, is yours to put back before the block ends.
- **Labels.** A label inside a block lives in the same assembly file as the rest of the program, so pick names that no function or other block uses.
- **Portability.** The block is x86-64 only. It will not work on another architecture.

## Not Supported Yet

- **Reading Monkey C variables.** A block cannot refer to a local variable by name.
- **Returning a value.** A block is a statement and has no value of its own.
- **Other architectures.** Only x86-64 assembly is accepted.

See also: [index](./index.md), it is a full index for all of the features this language supports.
