struct Test {
    int x = 5;
    int y = 6;

    fn add() -> int {
        return self::x + self::y;
    }
}

fn main() {
    Test a;
    print_int(a::add());
}
