fn fact(int n) -> int {
    if (n < 2) {
        return 1;
    }
    int m = n - 1;
    int r = fact(m);
    return n * r;
}

fn main() {
    int a = fact(1);
    print_int(a);
    int b = fact(5);
    print_int(b);
    int c = fact(6);
    print_int(c);
}
## expect: 1
## expect: 120
## expect: 720
