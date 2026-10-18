struct Math {
    int base = 10;

    fn plus(int v) -> int {
        return self::base + v;
    }

    fn scaled(int a ; int b) -> int {
        return self::base * a + b;
    }
}

fn main() {
    Math m;
    int five = 5;
    int r1 = m::plus(five);
    print_int(r1);

    int two = 2;
    int three = 3;
    int r2 = m::scaled(two ; three);
    print_int(r2);
}
## expect: 15
## expect: 23
