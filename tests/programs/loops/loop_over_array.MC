fn main() {
    int nums[3] = {10 , 20 , 30};
    int sum = 0;
    for(int i = 0, i < 3, i += 1) {
        sum += nums[i];
    }
    print_int(sum);
}
## expect: 60
