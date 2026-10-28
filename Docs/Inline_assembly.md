# Inline Assembly

> **Quick Summary:** Inline assembly (`asm -> size { ... }`) allows embedding raw x86-64 assembly instructions directly inside Monkey C functions or methods, reserving a specified stack space (rounded to the closest power of two). The compiler injects instructions directly into the output and automatically substitutes Monkey C local variable names with their corresponding stack slots (e.g. `mov edi, a` becomes `mov edi, [rbp - 8]`), leaving register preservation and label uniqueness up to the programmer.

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

## Referencing Local Variables

An inline assembly block can refer to Monkey C local variables directly by name. During code generation, the compiler resolves variable names to their allocated stack slot operand (e.g. `[rbp - 8]`):

```text
fn main() {
    int a = 7;
    asm -> 32 {
        mov edi, a
        call print_int
    }
}
```

The compiler automatically substitutes `a` with its stack location (such as `[rbp - 8]`). You can write either `mov edi, a` or `mov edi, [a]`; both resolve cleanly to the variable's stack slot.

## Things to Keep in Mind

The compiler does not read or check the assembly in a block. It issues a compile warning reminding you to verify instructions yourself:
```text
[Warning]: The Monkey C compiler does not verify asmembly, you must check it by your self and remeber that the asmbler will return an error if it is the case, but it will not go through this compiler
```

- **Registers.** The compiler's own code uses registers too. Anything you change that the surrounding code relies on, such as `rbp` or `rsp`, is yours to put back before the block ends.
- **Labels.** A label inside a block lives in the same assembly file as the rest of the program, so pick names that no function or other block uses.
- **Portability.** The block is x86-64 only. It will not work on another architecture.

## Not Supported Yet

- **Returning a value.** A block is a statement and has no value of its own.
- **Other architectures.** Only x86-64 assembly is accepted.

See also: [index](./index.md), it is a full index for all of the features this language supports.
