
fn get_commoner(gift: Option<&str>) {
	match gift {
		Some("chicken") => println!("chicken"),
		Some(inner) => println!("{}? How nice.", inner),
		None => println!("No gift? Oh well."),
	}
}

fn get_present(gift: Option<&str>) {
	let inside = gift.unwrap_or("snake");
	if inside == "snake" { panic!("AAAaaaaaa"); }
	println!("Yeah. I got a {}!", inside);
}

fn main() {
	let chicken = Some("chicken");
	let duck = Some("duck");
	let cow = "cow";
	let none = None;
	println!("run 18.2");
	get_commoner(chicken);
	get_commoner(duck);
	get_commoner(Some(cow));
	get_commoner(none);

	let bird = Some("robin");
	let nothing = None;
	get_present(bird);
	get_present(nothing);
}