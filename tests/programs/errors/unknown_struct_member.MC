struct SecondTest {
    int x = 7;
    int y = 8;
}

fn main() {
    SecondTest b;
    print_int(b::add());
}
## expect-error: 'add' is neither a field nor a method of struct 'SecondTest'
