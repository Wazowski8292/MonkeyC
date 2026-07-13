<div align="center">

# Monkey C

</div>

---

## Indexk

1. [How to Install and Run](#how-to-install-and-run)
2. [Token types](#token-types)
3. [Todo](#todo)
4. [Long term goals](#long-term-goals)
5. [AI](#AI)
6. [License](#license)
7. [Contributing](#contributing)

---

## How to Install and Run

### Prerequisites

- **Rust**
- **nasm** for compiling the assembly code, but another would also work.
- **gcc** for linking the compiled binary, but another would also work.
- **Monkey C code**

### Setup Instructions

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Magicchess1244/monkey_c.git
   cd monkey_c
   ```
2. **Compil the code**
    ```bash
    make run FILE=YourProgram.MC
    ```

---
## Token types
> Not every token is fully implemented yet

| Token | Lexeme | Description |
|---|---|---|
| `IF` | `if` | Conditional |
| `ELSE` | `else` | Fallback for `if` |
| `INT` | `int` | Integer |
| `FLOAT` | `float` | Float |
| `BOOL` | `bool` | Boolean |
| `CHAR` | `char` | Character |
| `STRING` | `string` | String |
| `PLUS` | `+` | Addition |
| `MINUS` | `-` | Subtraction |
| `MULTIPLICATION` | `*` | Multiplication |
| `DIVISION` | `/` | Division |
| `EQUALS` | `=` | Assignment |
| `LOGICAL_EQUALS` | `==` | Equality comparison |
| `LOGICAL_AND` | `&&` | Logical AND |
| `LOGICAL_OR` | `\|\|` | Logical OR |
| `NOT` | `!` | Logical negation |
| `RIGHT_BIT_SHIFT` | `>>` | Bitwise right shift |
| `LEFT_BIT_SHIFT` | `<<` | Bitwise left shift |
| `AND` | `&` | Bitwise AND |
| `OR` | `\|` | Bitwise OR |
| `INTEGER_LITERAL` | `0–9…` | Integer literal |
| `FLOAT_LITERAL` | `0–9.0…` | Floating point literal |
| `CHAR_LITERAL` | `'a'…` | Character literal |
| `STRING_LITERAL` | `"…"` | String literal |
| `BOOL_LITERAL` | `true`/`false` | Boolean literal |
| `FN` | `fn` | Function |
| `WHILE_LOOP` | `while` | While loop |
| `UNKNOW` | — | Unknown token |

---

## Todo

- [ ] Need to be able to pass reasingments as parameters and also function calls
- [ ] Do parameter type checks
- [ ] Add for loop
- [ ] Check if the operator make sense
- [ ] Precompile operations is posible
 
---

## Long tenm goals

- [ ] Add a simple standar library


## AI

During the development of this project AI has been use only to help gather information about kernels, make simple bash scripts, or add simple functions that have been **verifide by me**. Every other single line of code in this repository has been **writen by me** or has been copy and pasted from some amazing blogs that I have found online.

---

## License

This project is licensed under the **MIT License**.
See the `LICENSE` file for more details.

---

## Contributing

Contributions, ideas, and optimizations are welcome!
Feel free to open issues or submit pull requests.