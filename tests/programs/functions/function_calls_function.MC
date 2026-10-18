fn inner() -> int {
    return 7;
}

fn outer() -> int {
    int v = inner();
    return v + 1;
}

fn main() {
    int r = outer();
    print_int(r);
}
## expect: 8
