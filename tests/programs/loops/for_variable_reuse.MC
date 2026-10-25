fn main() {
    for(int x = 0, x < 2, x += 1) {
        print_int(x);
    }
    for(int x = 0, x < 2, x += 1) {
        print_int(x);
    }
}
## expect: 0
## expect: 1
## expect: 0
## expect: 1
