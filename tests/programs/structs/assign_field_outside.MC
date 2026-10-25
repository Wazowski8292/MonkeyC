struct Test {
    int x = 5;
    int y = 6;

    fn add() -> int {
        return self::x + self::y;
    }
}

fn main() {
    Test a;
    a::x = 20;
    print_int(a::x);
    int r = a::add();
    print_int(r);
}
## expect: 20
## expect: 26
