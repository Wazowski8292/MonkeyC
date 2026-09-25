# Token Types

A **token** is the smallest meaningful piece of source code that the lexer produces when it reads your Monkey C file. Before the compiler can understand a program, it has to break the raw text apart into these pieces — things like keywords, numbers, symbols, and identifiers — so the rest of the compiler doesn't have to deal with raw characters at all.

Not every token below is fully implemented yet. This page is meant to be a reference for what each one is and where it's used, not a guarantee that every one of them works today. If you'd rather see these in context first, the [language guide](./language-guide.md) walks through a small program that uses several of them.

## Keywords

Keywords are reserved words that have a fixed meaning in the language and can't be used as variable names.

| Token | Lexeme | Description |
|---|---|---|
| `IF` | `if` | Starts a conditional branch. |
| `FN` | `fn` | Declares a function. See [Functions](./language-guide.md#a-basic-program) in the language guide for an example. |
| `WHILE_LOOP` | `while` | Starts a while loop. See [While Loops](./language-guide.md#while-loops) in the language guide for an example. |

## Types

These tokens represent the built-in data types a variable or value can have.

| Token | Lexeme | Description |
|---|---|---|
| `INT` | `int` | A whole number. |
| `FLOAT` | `float` | A number with a decimal point. |
| `BOOL` | `bool` | A true/false value. |
| `CHAR` | `char` | A single character. |

## Literals

A literal is a value written directly in the source code, as opposed to a variable that holds a value.

| Token | Lexeme | Description |
|---|---|---|
| `INTEGER_LITERAL` | `0–9…` | A whole number written directly, e.g. `42`. |
| `FLOAT_LITERAL` | `0–9.0…` | A decimal number written directly, e.g. `3.14`. |
| `CHAR_LITERAL` | `'a'…` | A single character in quotes, e.g. `'a'`. |
| `STRING_LITERAL` | `"…"` | A sequence of characters in quotes, e.g. `"hello"`. |
| `BOOL_LITERAL` | `true`/`false` | A boolean value written directly. |

## Arithmetic Operators

Operators that perform math on numbers.

| Token | Lexeme | Description |
|---|---|---|
| `PLUS` | `+` | Addition. |
| `MINUS` | `-` | Subtraction. |
| `MULTIPLICATION` | `*` | Multiplication. |
| `DIVISION` | `/` | Division. |
| `PLUS_PLUS` | `++` | Increment a value by one. |
| `MINUS_MINUS` | `--` | Decrement a value by one. |

## Assignment Operators

Operators that store a value into a variable.

| Token | Lexeme | Description |
|---|---|---|
| `EQUALS` | `=` | Assigns a value to a variable. |
| `PLUS_EQUALS` | `+=` | Adds a value to a variable and stores the result. |
| `MINUS_EQUALS` | `-=` | Subtracts a value from a variable and stores the result. |

## Comparison Operators

Operators that compare two values and produce a boolean result.

| Token | Lexeme | Description |
|---|---|---|
| `LOGICAL_EQUALS` | `==` | True if both sides are equal. |
| `NOT_EQUALS` | `!=` | True if both sides are not equal. |
| `GREATER_THAN` | `>` | True if the left side is greater. |
| `LESS_THAN` | `<` | True if the left side is smaller. |
| `GREATER_THAN_EQUALS` | `>=` | True if the left side is greater than or equal. |
| `LESS_THAN_EQUALS` | `<=` | True if the left side is smaller than or equal. |

## Logical Operators

Operators that combine or invert boolean values.

| Token | Lexeme | Description |
|---|---|---|
| `LOGICAL_AND` | `&&` | True only if both sides are true. |
| `LOGICAL_OR` | `\|\|` | True if at least one side is true. |
| `NOT` | `!` | Flips a boolean value. |

## Bitwise Operators

Operators that work directly on the individual bits of a value, rather than its value as a whole.

| Token | Lexeme | Description |
|---|---|---|
| `RIGHT_BIT_SHIFT` | `>>` | Shifts bits to the right. |
| `LEFT_BIT_SHIFT` | `<<` | Shifts bits to the left. |
| `AND` | `&` | Bitwise AND. |
| `OR` | `\|` | Bitwise OR. |

## Other

| Token | Lexeme | Description |
|---|---|---|
| `UNKNOW` | — | Produced when the lexer runs into something it doesn't recognize. Useful for catching errors early. |

## Next Steps

- [Language Guide](./language-guide.md) — see these tokens used in an actual program.
- [Roadmap](./roadmap.md) — planned tokens and features not yet in this table, like structs and enums.
