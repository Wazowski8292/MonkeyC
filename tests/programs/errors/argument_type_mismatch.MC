fn add(int a; int b) -> int {
    return a + b;
}

fn main() {
    print_int(add(1 , 1.4f));
}
## expect-error: Type mismatch for argument 2 of function call
