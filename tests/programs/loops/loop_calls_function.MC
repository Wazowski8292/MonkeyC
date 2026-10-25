fn square(int n) -> int {
    return n * n;
}

fn main() {
    for(int i = 1, i < 5, i += 1) {
        int square = square(i)
        print_int(square);
    }
}
## expect: 1
## expect: 4
## expect: 9
## expect: 16
