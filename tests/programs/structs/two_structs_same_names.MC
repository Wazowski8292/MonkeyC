struct Test {
    int x = 5;
    int y = 6;

    fn add() -> int {
        return self::x + self::y;
    }
}

struct SecondTest {
    int x = 7;
    int y = 8;

    fn add() -> int {
        return self::x + self::y;
    }
}

fn main() {
    Test a;
    SecondTest b;

    print_int(a::add());
    print_int(b::add());
}
## expect: 11
## expect: 15
