struct Pair {
    int x = 1;
}

fn main() {
    Pair p;
    print_int(p::nope);
}
## expect-error: 'nope' is neither a field nor a method of struct 'Pair'
