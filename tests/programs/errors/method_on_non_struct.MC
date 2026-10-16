fn main() {
    int a = 5;
    print_int(a::add());
}
## ignore: an instance that is not a struct falls into Member::Unresolved, which reports no error
## expect-error: add
