fn add(int a , int b) -> int {
    return a + b;
}

fn main() {
    print_int(add(1));
}
## ignore: the semantic analyzer only checks for too many arguments, not too few
## expect-error: Expected: 2
