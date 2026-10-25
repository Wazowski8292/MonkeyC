struct Pair {
    int x = 5;
    int y = 6;
}

fn main() {
    Pair p;
    print_int(p::x);
    print_int(p::y);
}
## expect: 5
## expect: 6
