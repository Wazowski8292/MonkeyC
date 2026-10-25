fn add(int a ; int b) -> int {
    return a + b;
}

fn main() {
    int r1 = add(2 ; 3);
    print_int(r1);
    int r2 = add(10 ; 20);
    print_int(r2);
}
## expect: 5
## expect: 30
