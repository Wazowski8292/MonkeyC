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
    int r = a::add();
    print_int(r);
}
## expect: 11
