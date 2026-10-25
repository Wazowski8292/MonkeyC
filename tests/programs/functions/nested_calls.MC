fn twice(int n) -> int {
    return n * 2;
}

fn add(int a ; int b) -> int {
    return a + b;
}

fn main() {
    int s = add(3 ; 4);
    int t = twice(s);
    print_int(t);

    int one = 1;
    int two = 2;
    int x = twice(one);
    int y = twice(two);
    int z = add(x ; y);
    print_int(z);
}
## expect: 14
## expect: 6
