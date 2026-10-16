fn main() {
    for(int i = 0, i < 3, i += 1) {
        int inner = i;
    }
    print_int(inner);
}
## expect-error: Undefined symbol: inner
