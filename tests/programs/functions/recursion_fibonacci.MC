fn fib(int n) -> int {
    if (n < 2) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn main() {
    print_int(fib(0));
    print_int(fib(1));
    print_int(fib(10));
}
## expect: 0
## expect: 1
## expect: 55
