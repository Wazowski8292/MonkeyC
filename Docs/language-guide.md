# Language Guide

This page walks through the basic building blocks of Monkey C using a small example, explaining each part as it comes up. If you haven't compiled anything yet, start with [Getting Started](./getting-started.md); for a full list of keywords, operators, and literals, see [Token Types](./token-types.md).

## A Basic Program

Every Monkey C program needs a `main` function. A **function** is a named, reusable block of code — [`fn`](./token-types.md#keywords) is the keyword used to declare one, and `main` is the function that runs first when the program starts.

```c
fn main() {
    char text[5] = {'h', 'e', 'l', 'l', 'o'};

    int count = 0;
    while (count < 5) {
        print_char(text[count]);
        count++;
    }
}
```

Here's what each part of this example is doing.

### Arrays

```c
char text[5] = {'h', 'e', 'l', 'l', 'o'};
```

An **array** is a fixed-size collection of values of the same type, stored under one variable name and accessed by position. Here, `text` is an array of 5 [`char`](./token-types.md#types) values, initialized with the letters that spell "hello". The `[5]` states the array's size up front.

### Variables

```c
int count = 0;
```

A **variable** is a named piece of storage that holds a value which can change over time. `count` is declared as an [`int`](./token-types.md#types) (a whole number) and starts at `0`. It's used below to keep track of position while looping through the array.

### While Loops

```c
while (count < 5) {
    print_char(text[count]);
    count++;
}
```

A **while loop** repeats a block of code for as long as a condition stays true. Here, the condition is `count < 5`, so the loop keeps running until `count` reaches 5. Each time through, it prints the character at position `count` in the array, then increments `count` by one using [`++`](./token-types.md#arithmetic-operators).

### Putting It Together

Line by line, this program:

1. Creates an array holding the characters of "hello".
2. Sets up a counter starting at 0.
3. Loops five times, printing one character of the array on each pass.
4. Increases the counter after each print, until the loop condition is no longer true and the program ends.

This is a small example, but it touches most of the core pieces you'll use in any Monkey C program: functions, variables, arrays, and loops.

## Next Steps

- [Token Types](./token-types.md) — full reference for every keyword, operator, and literal used above and beyond.
- [Roadmap](./roadmap.md) — language features not shown here yet, like structs and enums.
