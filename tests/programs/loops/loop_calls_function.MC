fn square(int n) -> int {
    return n * n;
}

fn main() {
    for(int i = 1, i < 5, i += 1) {
        print_int(square(i));
    }
}
## expect: 1
## expect: 4
## expect: 9
## expect: 16
