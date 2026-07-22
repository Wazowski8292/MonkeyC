pub struct PubFuncs {
    pub name: &'static str,
    pub parameters: &'static [&'static str],
    pub function: &'static str,
}

pub const FUNCTIONS: &[PubFuncs] = &[
    PubFuncs {
        name: "print_int",
        parameters: &["int"],
        function:
"print_int:
    push rbp
    mov rbp, rsp
    mov esi, edi
    lea rdi, [rel fmt_int]
    xor eax, eax
    call printf
    pop rbp
    ret\n
    ",
    },
    PubFuncs {
        name: "print_float",
        parameters: &["float"],
        function:
"print_float:
    push rbp
    mov rbp, rsp
    lea rdi, [rel fmt_float]
    mov eax, 1
    call printf
    pop rbp
    ret\n
    ",
    },
    PubFuncs {
        name: "print_bool",
        parameters: &["bool"],
        function:
"print_bool:
    push rbp
    mov rbp, rsp
    lea rsi, [rel str_true]
    lea rax, [rel str_false]
    test edi, edi
    cmovz rsi, rax
    lea rdi, [rel fmt_bool]
    xor eax, eax
    call printf
    pop rbp
    ret\n
    ",
    },
    PubFuncs {
        name: "print_string",
        parameters: &["str"],
        function:
"print_string:
    push rbp
    mov rbp, rsp
    mov rsi, rdi
    lea rdi, [rel fmt_string]
    xor eax, eax
    call printf
    pop rbp
    ret\n
    ",
    },
    PubFuncs {
        name: "print_char",
        parameters: &["char"],
        function:
"print_char:
    push rbp
    mov rbp, rsp
    mov esi, edi
    lea rdi, [rel fmt_char]
    xor eax, eax
    call printf
    pop rbp
    ret\n
    ",
    },
];