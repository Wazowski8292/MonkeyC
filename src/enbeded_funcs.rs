pub struct PubFuncs {
    pub name: &'static str,
    pub function: &'static str,
}

pub const FUNCTIONS: &[PubFuncs] = &[
    PubFuncs {
        name: "print_int",
        function: 
"print_int:
    push rbp
    mov rbp, rsp
    mov esi, edi
    lea rdi, [rel fmt_int]
    xor eax, eax
    call printf
    pop rbp
    ret",
    },
];