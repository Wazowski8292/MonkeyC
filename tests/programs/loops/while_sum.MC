fn main() {
    int i = 0;
    int sum = 0;
    while (i < 5) {
        sum = sum + i;
        i++;
    }
    print_int(sum);
}
## expect: 10
