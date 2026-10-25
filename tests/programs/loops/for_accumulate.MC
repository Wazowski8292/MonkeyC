fn main() {
    int total = 0;
    for(int i = 1, i < 6, i += 1) {
        total += i;
    }
    print_int(total);
}
## expect: 15
