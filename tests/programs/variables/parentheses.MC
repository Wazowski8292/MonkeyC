fn main() {
    int a = (2 + 3) * 4;
    print_int(a);
}
## ignore: parentheses are flattened, so (2 + 3) * 4 is computed as 2 + 3 * 4 = 14
## expect: 20
