pub struct PubFuncs {
    pub name: &'static str,
    pub function: &'static str,
}

pub const FUNCTIONS: &[PubFuncs] = &[
    PubFuncs {
        name: "print_int",
        function: 
"print_int:\n
    push rbp\n
    mov rbp, rsp\n
    mov esi, edi\n
    lea rdi, [rel fmt_int]\n
    xor eax, eax\n
    call printf\n
    pop rbp\n
    ret\n",

    },
];