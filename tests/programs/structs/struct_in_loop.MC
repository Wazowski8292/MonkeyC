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
    for(int i = 0, i < 4, i += 1) {
        c::inc();
    }
    print_int(c::get());
}
## expect: 4
