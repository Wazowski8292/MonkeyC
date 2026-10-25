fn main() {
    int i = 0;
    while (i < 2) {
        int j = 0;
        while (j < 2) {
            int v = i * 10 + j;
            print_int(v);
            j++;
        }
        i++;
    }
}
## expect: 0
## expect: 1
## expect: 10
## expect: 11
