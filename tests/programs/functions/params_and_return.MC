fn add(int a , int b) -> int {
    return a + b;
}

fn main() {
    print_int(add(2 , 3));
    print_int(add(10 , 20));
}
## expect: 5
## expect: 30
