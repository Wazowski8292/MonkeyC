fn half() -> float {
    return 1.5f;
}

fn main() {
    int x = half();
}
## expect-error: Type mismatch: variable is 'int' but function 'half' returns 'float'
