# Token Types

> **Quick Summary:** Tokens are the atomic lexical units produced by the compiler lexer from raw source text. Monkey C categorizes tokens into reserved keywords (`fn`, `return`, `if`, `else`, `while`, `for`, `do`, `struct`, `asm`), primitive types (`int`, `float`, `double`, `str`, `bool`, `char`), literals, arithmetic/logical/bitwise operators, and delimiters (`::`, `->`).

A **token** is the smallest meaningful piece of source code that the lexer produces when it reads your Monkey C file. Before the compiler can understand a program, it has to break the raw text apart into these pieces — things like keywords, numbers, symbols, and identifiers — so the rest of the compiler doesn't have to deal with raw characters at all.

This page is a reference for what each token is and where it is used. If you'd rather see these in context first, the [language guide](./language-guide.md) walks through a small program that uses several of them.

## Keywords

Keywords are reserved words that have a fixed meaning in the language and can't be used as variable names.

| Token | Lexeme | Description |
|---|---|---|
| `FN` | `fn` | Declares a function. See [Functions](./language-guide.md#a-basic-program) in the language guide. |
| `RETURN` | `return` | Returns a value from a function or method. |
| `IF` | `if` | Starts a conditional branch. |
| `ELSE` | `else` | Alternative branch when an `if` condition evaluates to false. |
| `WHILE_LOOP` | `while` | Starts a while loop. See [While Loops](./language-guide.md#while-loops). |
| `FOR_LOOP` | `for` | Starts a counted for loop (syntax: `for(init, condition, step)`). |
| `DO_WHILE_LOOP` | `do` | Starts a do-while loop structure. |
| `STRUCT` | `struct` | Declares a struct type. See [Structs](./structs.md). |
| `ASSEMBLY` | `asm` | Starts an inline assembly block. See [Inline Assembly](./Inline_assembly.md). |

## Types

These tokens represent the built-in data types a variable or value can have.

| Token | Lexeme | Description |
|---|---|---|
| `INT` | `int` | A whole number (64-bit signed integer). |
| `FLOAT` | `float` | A single-precision floating point number (e.g. `3.14f`). |
| `DOUBLE` | `double` | A double-precision floating point number (e.g. `3.14`). |
| `STRING` | `str` | A string of characters. |
| `BOOL` | `bool` | A true/false value. |
| `CHAR` | `char` | A single character. |

## Literals

A literal is a value written directly in the source code, as opposed to a variable that holds a value.

| Token | Lexeme | Description |
|---|---|---|
| `INTEGER_LITERAL` | `0–9…` | A whole number written directly, e.g. `42`. |
| `FLOAT_LITERAL` | `0–9.0…f` | A single-precision float literal ending in `f`/`F`, e.g. `3.14f`. |
| `DOUBLE_LITERAL` | `0–9.0…` | A double-precision decimal number written directly, e.g. `3.14`. |
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

## Delimiters and Punctuation

Symbols used for structuring expressions, member access, and declarations.

| Token | Lexeme | Description |
|---|---|---|
| `DOTS` | `::` | Struct field and method access operator (e.g. `self::x`, `a::add()`). |
| `RETURN_TYPE` | `->` | Delimiter for function return types (`fn f() -> int`) and inline assembly allocation (`asm -> 32`). |

## Other

| Token | Lexeme | Description |
|---|---|---|
| `UNKNOW` | — | Produced when the lexer runs into something it doesn't recognize. Useful for catching errors early. |

## Next Steps

- [Language Guide](./language-guide.md) — see these tokens used in an actual program.
- [Roadmap](./roadmap.md) — planned features and roadmap goals.
