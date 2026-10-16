fn helper() -> int {
    return secret;
}

fn main() {
    int secret = 5;
    print_int(helper());
}
## ignore:
## expect-error: Undefined symbol: secret
