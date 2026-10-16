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
    print_int(a::get());
    print_int(b::get());
}
## expect: 2
## expect: 1
