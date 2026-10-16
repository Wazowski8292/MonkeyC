fn main() {
    print_int(later());
}

fn later() -> int {
    return 9;
}
## ignore: functions must currently be defined before they are called (analyzer reports "Undefined symbol: later")
## expect: 9
