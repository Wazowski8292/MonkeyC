struct Math {
    int base = 10;

    fn plus(int v) -> int {
        return self::base + v;
    }

    fn scaled(int a , int b) -> int {
        return self::base * a + b;
    }
}

fn main() {
    Math m;
    print_int(m::plus(5));
    print_int(m::scaled(2 , 3));
}
## expect: 15
## expect: 23
