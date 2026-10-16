fn twice() -> int {
    return 1;
}

fn twice() -> int {
    return 2;
}

fn main() {
    print_int(twice());
}
## ignore: the semantic analyzer has no duplicate-function check yet
## expect-error: already
