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
    print_int(compute());
    print_int(compute());
}
## ignore:
## expect: 11
## expect: 11

