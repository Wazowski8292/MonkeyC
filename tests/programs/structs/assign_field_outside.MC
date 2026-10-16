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
    print_int(a::add());
}
## expect: 20
## expect: 26
