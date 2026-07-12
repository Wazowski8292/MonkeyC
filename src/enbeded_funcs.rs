pub struct PubFuncs {
    name: &str,
    function: String,
}

const FUNTIONS = PubFuncs {
    {
       name: "print_int",
       function: "print_int:
                    push rbp
                    mov rbp, rsp
                    mov esi, edi
                    lea rdi, [rel fmt_int]
                    xor eax, eax
                    call printf
                    pop rbp
                    ret",
    }
}
