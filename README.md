<div align="center">

# Monkey C

</div>

---

## Indexk

1. [How to Install and Run](#how-to-install-and-run)
2. [Token types](#token-types)
3. [Todo](#todo)
4. [Long term goals](#long-term-goals)
5. [Example](#example)
6. [AI](#AI)
7. [License](#license)
8. [Contributing](#contributing)

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
| `INT` | `int` | Integer |
| `FLOAT` | `float` | Float |
| `BOOL` | `bool` | Boolean |
| `CHAR` | `char` | Character |
| `PLUS` | `+` | Addition |
| `MINUS` | `-` | Subtraction |
| `MULTIPLICATION` | `*` | Multiplication |
| `DIVISION` | `/` | Division |
| `EQUALS` | `=` | Assignment |
| `PLUS_EQUALS` | `+=` | Addition assignment |
| `MINUS_EQUALS` | `-=` | Subtraction assignment |
| `PLUS_PLUS` | `++` | Increment by one |
| `MINUS_MINUS` | `--` | Decrement by one |
| `LOGICAL_EQUALS` | `==` | Equality comparison |
| `NOT_EQUALS` | `!=` | Not equals comparison |
| `GREATER_THAN` | `>` | Greater than comparison |
| `LESS_THAN` | `<` | Less than comparison |
| `GREATER_THAN_EQUALS` | `>=` | Greater than or equals comparison |
| `LESS_THAN_EQUALS` | `<=` | Less than or equals comparison |
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

- [ ] Add structs
- [ ] Add function realated to the structs
- [ ] Add enums
- [ ] Add for loop
- [ ] Check if the operator make sense
- [ ] Precompile operations is posible
 
---

## Long tenm goals

- [ ] Add a function that says how many elemnts does an array have
- [ ] Make enum store values, like rust
- [ ] Add heap
- [ ] Add vectors
- [ ] Add helper functions to vectors
- [ ] Add multithreadding

## Example 

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
