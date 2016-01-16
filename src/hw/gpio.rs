//
// imports
//
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write, Read};
use std::result;

pub type Result<T> = result::Result<T, &'static str>;

//
// enums
//
pub enum Direction { In, Out }
pub enum State { Low, High }

//
// Pin struct - represents a GPIO pin
//
pub struct Pin {
	port : u8,
	file : File,
}

impl Pin {
	//
	// open(port, direction) - opens and returns a Pin representing a GPIO pin
	//
	pub fn open(port: u8, direction: Direction) -> Result<Pin> {
		// get path to /sys files for this pin
		let value_path = format!("/sys/class/gpio/gpio{}/value", port);
		let direction_path = format!("/sys/class/gpio/gpio{}/direction", port);

		/*
		let md = metadata(&format!("/sys/class/gpio/gpio{}", port));
		if md.is_err() {
			let mut export_file = OpenOptions::new().write(true).open("/sys/class/gpio/export").unwrap();
			try!(write!(export_file, "{}", port).or(Err("unable to export gpio")));
		}
		*/

		// open the value and directoin files
		let mut value_file = OpenOptions::new().write(true).open(value_path.clone()).unwrap();
		let mut direction_file = OpenOptions::new().write(true).open(direction_path).unwrap();

		let dir = match direction {
			Direction::In => "in",
			Direction::Out => "out"
		};

		// set the direction
		try!(write!(direction_file, "{}", dir).or(Err("unable to set gpio direction")));

		// if direction is output, set the value to Low
		match direction {
			Direction::In => (),
			Direction::Out => try!(write!(value_file, "{}", "0").or(Err("unable to set gpio value")))
		};

		// return the pin
		Ok(Pin {
			port: port,
			file: value_file,
		})
	}

	//
	// get_state() - get the state of an input GPIO pin
	//
	pub fn get_state(&mut self) -> Result<State> {
		try!(self.file.seek(SeekFrom::Start(0)).or(Err("unable to seek gpio fd")));
		
		let mut buf = [0u8];
		let amount = try!(self.file.read(&mut buf).or(Err("unable to read from gpio")));
		
		if amount == 0 {
			Err("no state read from pin")
		} else if buf[0] == b'0' {
			Ok(State::Low)
		} else if buf[0] == b'1' {
			Ok(State::High)
		} else {
			Err("unexpected state read from pin")
		}
	}
	
	//
	// set_state(state) - set the state of an output GPIO pin
	//
	pub fn set_state(&mut self, state : State) -> Result<()> {
		let buf = match state { State::Low => b"0" , State::High => b"1" };
		try!(self.file.seek(SeekFrom::Start(0)).or(Err("unable to seek gpio fd")));
		try!(self.file.write(buf).or(Err("unable to set gpio state")));
		Ok(())
	}
}

/*
impl Drop for Pin {
	fn drop(&mut self) {
		let mut unexport_file = OpenOptions::new().write(true).open("/sys/class/gpio/unexport").unwrap();
		drop(write!(unexport_file, "{}", self.port).or(Err("unable to unexport gpio")))
	}
}
*/
