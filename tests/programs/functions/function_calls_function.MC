fn inner() -> int {
    return 7;
}

fn outer() -> int {
    return inner() + 1;
}

fn main() {
    print_int(outer());
}
## expect: 8
