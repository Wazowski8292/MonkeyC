# Roadmap

This page lists what's planned for Monkey C, split into near-term work and longer-term goals. Nothing here is a promise of a timeline, it's just a record of where the language is headed. For what's already implemented, see the [token reference](./token-types.md) and [language guide](./language-guide.md).

## Todo

These are the next things planned for the language itself.

- **Structs** — a struct is a custom type that groups several values together under one name, so related data can be passed around as a single unit instead of separate variables.
- **Struct-related functions** — functions that are tied to a struct, letting you organize behavior alongside the data it operates on.
- **Enums** — an enum is a type made up of a fixed set of named values, useful for representing something that can only be one of a few known states.
- **For loops** — a loop construct built for the common case of repeating something a known number of times, as an alternative to writing that logic with a `while` loop.
- **Operator sanity checking** — validation that an operator is actually being used on types it makes sense for, e.g. catching an attempt to add a `bool` and a `char`.
- **Precompiled operations** — evaluating parts of an expression at compile time when possible, instead of at runtime, so the resulting program has less work to do.

## Long Term Goals

These are bigger, further-out goals for the language and its standard library.

- **Array length lookup** — a built-in way to ask how many elements an [array](./language-guide.md#arrays) holds, rather than tracking that separately yourself.
- **Enums with stored values** — enums that can carry data along with each variant, similar to how Rust's enums work, rather than just being a plain named value.
- **Heap allocation** — support for allocating memory that outlives the function it was created in, as opposed to memory that's automatically cleaned up when a function returns.
- **Vectors** — a growable, resizable array type, for when the number of elements isn't known ahead of time.
- **Vector helper functions** — common operations on vectors (things like adding, removing, or searching for elements) built into the language or its standard library.
- **Multithreading** — the ability to run more than one sequence of instructions at the same time, letting a program do multiple things concurrently.
