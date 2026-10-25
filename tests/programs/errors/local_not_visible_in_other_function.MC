fn helper() -> int {
    return secret;
}

fn main() {
    int secret = 5;
    print_int(helper());
}
## expect-error: Undefined symbol: secret
