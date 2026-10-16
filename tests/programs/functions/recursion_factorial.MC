fn fact(int n) -> int {
    if (n < 2) {
        return 1;
    }
    return n * fact(n - 1);
}

fn main() {
    print_int(fact(1));
    print_int(fact(5));
    print_int(fact(6));
}
## expect: 1
## expect: 120
## expect: 720
