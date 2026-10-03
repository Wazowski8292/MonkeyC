struct Point {
	int x = 50;
	int y = 60;
}

fn Num () -> int {
	return 5;
}

fn main() {
	Point p;
	
	p::x = 70;

	print_int(p::x);
	print_int(Num());
}
