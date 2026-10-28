# Standard Library & Language Index

> **Quick Summary:** A central reference indexing all built-in functions, types, and language features in Monkey C, providing quick links to their dedicated documentation pages.

This page is a flat list of everything in the language and standard library, so a specific function, type, or feature can be found without digging through every doc. Each entry links straight to the page that explains what it is and how it works.

## Built-in Functions

| Name | Description |
|---|---|
| [`print`](./print.md) | Writes a value to standard output. Covers `print_int`, `print_float`, `print_bool`, `print_char`, and `print_string`. |
| [Adding Built-in Functions](./adding-builtin-functions.md) | Guide for defining new built-in functions via `PubFuncs` and assembly in the compiler. |

## Types & Features

| Name | Description |
|---|---|
| [`structs`](./structs.md) | Bundles fields and associated methods into a custom type allocated on the stack. |
| [**Inline assembly**](./Inline_assembly.md) | Special block written as `asm -> size { ... }` that embeds raw x86-64 assembly instructions and resolves local variable stack offsets. |
| [Token Types](./token-types.md) | Complete reference of keywords, types, literals, operators, and delimiters recognized by Monkey C. |
| [Language Guide](./language-guide.md) | Walkthrough of core syntax, variables, arrays, loops, and functions. |
