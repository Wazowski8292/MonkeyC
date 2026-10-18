struct Test {
    int x = 5;
    int y = 6;

    fn add() -> int {
        return self::x + self::y;
    }
}

fn compute() -> int {
    Test t;
    int r = t::add();
    return r;
}

fn main() {
    int r = compute();
    print_int(r);
    int r2 = compute();
    print_int(r2);
}
## expect: 11
## expect: 11
