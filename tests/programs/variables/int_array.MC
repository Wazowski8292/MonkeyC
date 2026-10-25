fn main() {
    int nums[3] = {10 , 20 , 30};
    print_int(nums[0]);
    int s = nums[1] + nums[2];
    print_int(s);
    nums[1] = 5;
    print_int(nums[1]);
}
## expect: 10
## expect: 50
## expect: 5
