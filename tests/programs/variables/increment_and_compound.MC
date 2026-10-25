fn main() {
    int i = 5;
    i++;
    print_int(i);

    int x = 5;
    x += 3;
    print_int(x);
    x -= 2;
    print_int(x);
}
## expect: 6
## expect: 8
## expect: 6
