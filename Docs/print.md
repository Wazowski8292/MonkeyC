# print

`print` is the standard library's function for writing a value to the screen. There isn't a single `print` in the assembly itself — instead there's one variant per type, each named `print_<type>`, and the compiler picks the right one based on the type of the value you're printing.

A **standard library function** is one that ships with the language and is always available to a Monkey C program, without needing to be written or imported. Every print variant is hand-written directly in x86-64 assembly rather than compiled from Monkey C source, and all of them work the same way underneath: take one argument, format it, and hand it to `printf` to actually write out.

## print_int

Prints an `int`.

```asm
print_int:
    push rbp
    mov rbp, rsp
    mov esi, edi
    lea rdi, [rel fmt_int]
    xor eax, eax
    call printf
    pop rbp
    ret
```

The value comes in through `edi`, following the standard calling convention where the first integer argument lands in that register. It's moved into `esi` to become the second argument to `printf`, while `rdi` is set to point at a format string (`fmt_int`) telling `printf` to expect and print an integer.

## print_float

Prints a `float`.

```asm
print_float:
    push rbp
    mov rbp, rsp
    cvtss2sd xmm0, xmm0
    lea rdi, [rel fmt_float]
    mov eax, 1
    call printf
    pop rbp
    ret
```

Floating point values are passed in `xmm0` rather than a general-purpose register. Since `printf` expects a `double` for its floating point formatting, `cvtss2sd` converts the incoming single-precision float into a double before the call. Setting `eax` to `1` tells `printf` (per the System V calling convention) that one vector register is being used for arguments.

## print_bool

Prints a `bool` as either `true` or `false`.

```asm
print_bool:
    push rbp
    mov rbp, rsp
    lea rsi, [rel str_true]
    lea rax, [rel str_false]
    test edi, edi
    cmovz rsi, rax
    lea rdi, [rel fmt_bool]
    xor eax, eax
    call printf
    pop rbp
    ret
```

Both possible strings (`str_true` and `str_false`) are loaded up front. `test edi, edi` checks whether the incoming boolean value is zero, and `cmovz` conditionally replaces `rsi` with the "false" string only if it was. This avoids a branch — the CPU always does the same work, just picks between two already-loaded values.

## print_char

Prints a single `char`.

```asm
print_char:
    push rbp
    mov rbp, rsp
    mov esi, edi
    lea rdi, [rel fmt_char]
    xor eax, eax
    call printf
    pop rbp
    ret
```

Structurally identical to `print_int` — the character's value is passed straight through to `printf` using a character format string.

## print_string

Prints a `str`.

```asm
print_string:
    push rbp
    mov rbp, rsp
    mov rsi, rdi
    lea rdi, [rel fmt_string]
    xor eax, eax
    call printf
    pop rbp
    ret
```

The incoming argument is already a pointer to the string's data, so it's passed straight through as the value `printf` will print with its string format specifier.

## Shared Pattern

Every print variant follows the same shape:

1. Set up a stack frame with `push rbp` / `mov rbp, rsp`.
2. Move the incoming argument into the register `printf` expects it in.
3. Load the address of the right format string into `rdi`.
4. Call `printf`.
5. Tear down the stack frame with `pop rbp` and return with `ret`.

For how these variants are registered in the compiler itself, and what's needed to add a new one, see [Adding Built-in Functions](./adding-builtin-functions.md).
