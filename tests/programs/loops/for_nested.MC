fn main() {
    for(int i = 0, i < 2, i += 1) {
        for(int j = 0, j < 2, j += 1) {
            int v = i * 10 + j;
            print_int(v);
        }
    }
}
## expect: 0
## expect: 1
## expect: 10
## expect: 11
