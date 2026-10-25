fn twice() -> int {
    return 1;
}

fn twice() -> int {
    return 2;
}

fn main() {
    print_int(twice());
}
## expect-error: already
