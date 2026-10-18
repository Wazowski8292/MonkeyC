struct Counter {
    int n = 0;

    fn inc() {
        self::n = self::n + 1;
    }

    fn get() -> int {
        return self::n;
    }
}

fn main() {
    Counter a;
    Counter b;
    a::inc();
    a::inc();
    b::inc();
    int ra = a::get();
    print_int(ra);
    int rb = b::get();
    print_int(rb);
}
## expect: 2
## expect: 1
