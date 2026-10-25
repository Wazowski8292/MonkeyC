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

    int ra = a::add();
    print_int(ra);
    int rb = b::add();
    print_int(rb);
}
## expect: 11
## expect: 15
