struct Test {
    int x = 5;
    int y = 6;

    fn add() -> int {
        int x = self::x;
        int y = self::y;
        return x + y;
    }
}

fn main() {
    Test a;
    print_int(a::add());
}
## expect: 11
