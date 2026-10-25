fn main() {
    int a = 1;
    print_int(a);
    a = 99;
    print_int(a);
    a = a + 1;
    print_int(a);
}
## expect: 1
## expect: 99
## expect: 100
