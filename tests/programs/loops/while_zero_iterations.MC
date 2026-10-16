fn main() {
    int i = 10;
    while (i < 5) {
        print_int(i);
        i++;
    }
    print_int(0);
}
## expect: 0
