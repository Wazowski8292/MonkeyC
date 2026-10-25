fn add(int a , int b) -> int {
    return a + b;
}

fn main() {
    int r = add(1);
    print_int(r);
}
## expect-error: Expected: 2
