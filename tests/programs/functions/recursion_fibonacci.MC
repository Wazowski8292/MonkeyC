fn fib(int n) -> int {
    if (n < 2) {
        return n;
    }
    int m1 = n - 1;
    int m2 = n - 2;
    int a = fib(m1);
    int b = fib(m2);
    return a + b;
}

fn main() {
    int r0 = fib(0);
    print_int(r0);
    int r1 = fib(1);
    print_int(r1);
    int r10 = fib(10);
    print_int(r10);
}
## expect: 0
## expect: 1
## expect: 55
