use colored::{ColoredString, Colorize};

pub(crate) struct Table {
	state: Vec<Vec<ColoredString>>,
	cols: usize,
}

impl Table {
	pub(crate) fn new(cols: usize) -> Table {
		Self {
			state: vec![vec![]],
			cols,
		}
	}

	pub(crate) fn push_cell(&mut self, input: ColoredString) {
		// unwraps: last/last_mut are guaranteed to exist on the top-level
		// vector since they are declared in the initializer
		let mut this_str: ColoredString = input.trim_matches('\0').into();
		if self.state.len() == 1
			&& self.state.last().unwrap().len() != self.cols
		{
			this_str = this_str.bold();
		}

		if self.state.last().unwrap().len() < self.cols {
			self.state.last_mut().unwrap().push(this_str)
		} else {
			self.state.push(vec![this_str])
		}
	}

	pub(crate) fn push_cells(&mut self, input: Vec<ColoredString>) {
		for each in input.into_iter() {
			self.push_cell(each);
		}
	}

	pub(crate) fn draw(&self) {
		let mut col_widths = vec![0usize; self.cols];

		// For row in table, for col in enumerate(row cells), if col width in
		// arr is smaller, set it to cell width
		for row in self.state.iter() {
			for (col, cell) in row.iter().enumerate() {
				if col_widths[col] < cell.len() {
					col_widths[col] = cell.len();
				}
			}
		}

		// Draw top
		print!("╭");
		// Clone :(
		for (col, width) in col_widths.clone().into_iter().enumerate() {
			for _ in 0..width + 2 {
				print!("─");
			}
			if col != self.cols - 1 {
				print!("┬");
			}
		}
		print!("╮");
		// Draw content
		for row in self.state.iter() {
			print!("\n│ ");
			for (idx, cell) in row.iter().enumerate() {
				print!("{}", cell);
				for _ in 0..(col_widths[idx] - cell.len()) {
					print!(" ");
				}
				if idx != row.len() {
					print!(" │ ")
				}
			}
		}
		// Draw bottom
		print!("\n╰");
		for (col, width) in col_widths.into_iter().enumerate() {
			for _ in 0..width + 2 {
				print!("─");
			}
			if col != self.cols - 1 {
				print!("┴");
			}
		}
		println!("╯");
	}
}
