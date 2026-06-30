<div align="center">

# MonkeyC

</div>

---

## Index

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
- **Code**

### Setup Instructions

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Magicchess1244/MonkeyC.git
   cd MonkeyC
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
| `STRING` | `string` | String |
| `PLUS` | `+` | Addition |
| `MINUS` | `-` | Subtraction |
| `EQUALS` | `=` | Assignment |
| `INTEGER_LITERAL` | `0–9…` | Interger literal |
| `FLOAT_LITERAL` | `0–9.0…` | Floating point integers literal |
| `FN` | `fn` | Function |
| `UNKNOW` | — | Unknown token |

---

## Todo

- [x] Add a parser
- [x] Make semantic analyzer detect var defenitions
- [x] Make semantic analyzer detect funtion defenitions
- [x] Make semantic analyzer reasing and call var defenitions
- [x] Need to fix a problem with finding fn definitions
- [ ] Make function parameter actually do something(do checks)
    -> [x] Do parameter amount checks
    -> [ ] Do parameter type checks
- [ ] Add conditionals

---

## Long tenm goals

- [ ] Rewrite the hole TypeTable but in a simple programming language similar to assembly
- [ ] Pass the code to assembly
- [ ] Actually compile
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