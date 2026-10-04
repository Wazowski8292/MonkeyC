# Structs

A **struct** groups related values (its *fields*) and the functions that work on them (its *methods*) under one name. Each instance of a struct keeps its own copy of the fields.

```text
struct Test {
    int x = 5;
    int y = 6;

    fn add() -> int {
        return self::x + self::y;
    }
}

fn main() {
    Test a;
    print_int(a::add());
}
```

This program defines `Test`, creates an instance named `a`, calls its method and prints `11`.

## Defining a Struct

A struct is declared with the `struct` keyword, a name and a body in braces. The body holds fields and methods in any order.

| Part | Description |
| --- | --- |
| `struct Name { ... }` | Declares a new type called `Name`. |
| `type field = default;` | A field with a type, a name and an optional default value. |
| `fn method() -> type { ... }` | A method: a function that also receives the instance it was called on. |

## Fields

Fields are declared like ordinary variables. Every field with a default value is set to it when an instance is created. A field without a default is left uninitialized, so assign it before reading it.

Inside a method, a field is read through `self`. Outside, use the name of the instance:

```text
self::x
a::x
```

Fields are only reachable through `::`, so a local variable can share its name with a field without clashing:

```text
fn add() -> int {
    int x = self::x;
    int y = self::y;
    return x + y;
}
```

## Instances

An instance is declared with the struct name followed by the variable name:

```text
Test a;
```

This reserves space for every field on the stack and fills in the defaults. The instance lives until the function that declared it returns.

## Methods

A method is called on an instance with `::` and can be used anywhere a normal call can:

```text
print_int(a::add());
```

Inside a method, `self` is the instance the method was called on. You never declare it: the compiler adds it as the first parameter, and any parameters you write come after it. Since `self` is a pointer, a method that changes a field changes the caller's instance, not a copy.

## Using Several Structs

Each struct has its own fields and methods, so different structs can reuse the same names. A call is matched to the struct of the instance it is made on.

```text
struct SecondTest {
    int x = 7;
    int y = 8;

    fn add() -> int {
        return self::x + self::y;
    }
}

fn main() {
    Test a;
    SecondTest b;

    print_int(a::add());
    print_int(b::add());
}
```

Together with `Test` from the first example, this prints `11` and then `15`.

Using a name the struct does not have is a compile error:

```text
'add' is neither a field nor a method of struct 'SecondTest'
```

## How Structs Work

You do not need this section to use structs, but it explains the behavior above.

**Memory layout.** An instance is stored like a small array. Each field takes one 8-byte slot, in declaration order, so `Test` looks like this:

| Offset | Field | Initial value |
| --- | --- | --- |
| `0` | `x` | `5` |
| `8` | `y` | `6` |

The compiler translates a field name to its position, so `self::x` becomes `self[0]` and `self::y` becomes `self[1]`.

**Methods are ordinary functions.** Each method is emitted under its struct's name, so `add` in `Test` becomes `Test.add`. A `.` cannot appear in a Monkey C identifier, so this name can never collide with a function you write, and two structs can both have an `add`.

**The `self` pointer.** `self` arrives in `rdi`, following the standard calling convention. Reading a field loads that pointer and adds the field's offset:

```asm
mov [rbp - 8], rdi
mov rax, [rbp - 8]
mov rbx, 0
imul rbx, 8
add rax, rbx
mov rcx, [rax]
```

The pointer is stored, loaded back, offset by `0 * 8` to reach the first field, and the value at that address is read. A call like `a::add()` passes the address of `a` in `rdi`.

## Not Supported Yet

- **Constructors.** Default values on the fields are the only way to set initial values.
- **Static methods.** Methods can only be called on an instance.
- **Field sizes.** Every field uses a full 8-byte slot, whatever its type.

See also: [index](./index.md), it is a full index for all of the features this language supports.
