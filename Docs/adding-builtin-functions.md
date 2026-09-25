# Adding Built-in Functions

This page explains the internal structure used to define built-in functions like `print_int` and `print_string` (see [print](./print.md)), and what's needed to add a new one.

## The PubFuncs Struct

Every built-in function is described by one instance of a Rust struct called `PubFuncs`:

```rust
pub struct PubFuncs {
    pub name: &'static str,
    pub parameters: &'static [&'static str],
    pub function: &'static str,
}
```

A **struct**, in this context, is a Rust type that groups a few related pieces of data together under one name — here, everything the compiler needs to know about one built-in function. `PubFuncs` has three fields:

- **`name`** — the identifier a Monkey C program uses to call the function, e.g. `"print_int"`.
- **`parameters`** — the list of Monkey C types the function accepts, in order. Most built-ins so far take exactly one, e.g. `&["int"]`.
- **`function`** — the actual x86-64 assembly implementation, stored as a raw string that gets embedded directly into the compiled output.

All three fields are `&'static`, meaning they point to data that lives for the entire lifetime of the program rather than being allocated at runtime. This is what lets every `PubFuncs` value be defined as a plain constant rather than something built up when the compiler starts.

## The FUNCTIONS List

Every built-in function gets collected into a single constant array:

```rust
pub const FUNCTIONS: &[PubFuncs] = &[
    PubFuncs {
        name: "print_int",
        parameters: &["int"],
        function: "print_int:\n    push rbp\n    ...",
    },
    // one entry per built-in function
];
```

This is the list the compiler walks through to know which functions are built into the language, what they're called, what they accept, and what assembly to emit for each one.

## Characteristics of a Valid Entry

For a `PubFuncs` entry to work correctly, it needs to follow a few rules:

- **`name` must be unique** across the whole list, and should match the name a Monkey C program will actually call.
- **`parameters` must match the assembly**, in both count and order. If the assembly expects a value in `edi`, `parameters` should list exactly one type in that position.
- **`function` must be valid, complete assembly**, starting with a label matching `name` and ending in `ret`. It needs to follow the calling convention correctly (arguments in the right registers, stack frame set up and torn down) since nothing else on the compiler side checks this for you.

## Adding a New Built-in Function

1. Write and test the assembly implementation on its own first, to confirm it does what you want.
2. Add a new `PubFuncs` entry to the `FUNCTIONS` array, filling in `name`, `parameters`, and `function`.
3. Give it its own doc page (like [print.md](./print.md)) explaining what it does and how, following the same format as the existing entries.
4. Add a link to that page in [the standard library index](./index.md).
