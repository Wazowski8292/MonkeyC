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
    Counter c;
    c::inc();
    c::inc();
    c::inc();
    int r = c::get();
    print_int(r);
}
## expect: 3
