fn one(int a) -> int {
    return a;
}

fn main() {
    print_int(one(1 , 2 , 3));
}
## expect-error: Too many arguments for function call
