struct Calc {
    int a = 3;
    int b = 4;

    fn sum() -> int {
        return self::a + self::b;
    }

    fn sum_again() -> int {
        int s = self::sum();
        return s;
    }
}

fn main() {
    Calc c;
    print_int(c::sum_again());
}
## ignore:
## expect: 7
