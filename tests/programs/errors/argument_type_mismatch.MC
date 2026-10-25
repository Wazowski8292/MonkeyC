fn add(int a; int b) -> int {
    return a + b;
}

fn main() {
    int a = add(1; 1.5f);
    print_int(a);
}
## expect-error: Type mismatch for argument 2 of function call
