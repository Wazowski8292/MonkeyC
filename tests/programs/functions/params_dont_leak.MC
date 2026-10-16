fn change(int n) -> int {
    n = n + 100;
    return n;
}

fn main() {
    int a = 1;
    print_int(change(a));
    print_int(a);
}
## expect: 101
## expect: 1
