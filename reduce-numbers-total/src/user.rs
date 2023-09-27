pub const MAX_NUMBERS: usize = 3;

pub struct User {
	pub numbers_added: Vec<i8>,
}

impl User {
	pub fn total_numbers(&self) -> i8 {
			let initial: i8 = 0;

			self.numbers_added.iter().fold(
					initial, 
					|accumulator: i8, current: &i8| {
					accumulator + current
			})
	}
}