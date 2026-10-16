fn twice(int n) -> int {
    return n * 2;
}

fn add(int a , int b) -> int {
    return a + b;
}

fn main() {
    print_int(twice(add(3 , 4)));
    print_int(add(twice(1) , twice(2)));
}
## expect: 14
## expect: 6
