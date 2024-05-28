
struct Person {
	job: Option<Job>,
}

#[derive(Debug, Clone, Copy)]
struct Job {
	phone_number: Option<PhoneNum>,
}

#[derive(Debug, Clone, Copy)]
struct PhoneNum {
	area_code: Option<u8>,
	number: u32,
}

impl Person {
	fn work_phone_area_code(&self) -> Option<u8> {
		self.job?.phone_number?.area_code
	}

	fn work_phone_num(&mut self) -> Option<u32> {
		self.job = Some(Job {
			phone_number: Some(PhoneNum {
				area_code: Some(12),
				number: 1245,
			}),
		});
		self.job?.phone_number?.number = 135;
		Some(self.job?.phone_number?.number)
	}
}

impl PhoneNum {
	fn get_number(&mut self) -> u32 {
		self.number = 321;
		self.number
	}
}

fn main() {
	println!("run 18.2");
	let mut person = Person {
		job: Some(Job {
			phone_number: Some(PhoneNum {
				area_code: Some(61),
				number: 439517,
			}),
		}),
	};
	person.job = Some(Job {
		phone_number: Some(PhoneNum {
			area_code: Some(62),
			number: 124,
		}),
	});
	let person2 = Person { job: None};
	println!("Area code is {}", person2.work_phone_area_code().unwrap_or(0));
	println!("Phone number is {}", person.work_phone_num().unwrap_or(0));

	let mut pnum = PhoneNum {
		area_code: Some(61),
		number: 123456,
	};
	println!("Mutable Phone number is {} num: {}", pnum.get_number(), pnum.number);
}