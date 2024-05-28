
fn print_type_of<T>(_: &T) {
	println!("{}", std::any::type_name::<T>())
}

fn main() {
	let logical: bool = false;
	let logi_true: bool = true;

	if logical {
		println!("true");
	} else {
		println!("false");
	}
	println!("logi_true is {}", logi_true);

	let a_float: f64 = 1.0;
	let an_integer = 5i32;

	println!("a_float is {:?} {}", a_float, an_integer);

	let mut inferred_type = 12;
	println!("inferred_type is {}", inferred_type);
	inferred_type = 4294967296i64;
	print_type_of(&inferred_type);

	let mut mutable = 12;
	mutable = 21;

	// 报错，不能改变类型
	// mutable = true;

	let mutable = true;
}