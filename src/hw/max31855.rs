//
// imports
//
use std::fmt;
use std::fs::File;
use std::path::Path;
use std::result;
use std::io::Read;

pub type Result<T> = result::Result<T, &'static str>;

//
// Thermocouple struct
//
pub struct Thermocouple {
	spi: File, // the spidev interface to this thermocouple
}

//
// Sample struct
//
#[allow(dead_code)]
pub struct Sample {
	oc_fault: bool,               // open (no connections)
	scg_fault: bool,              // short-circuited to GND
	scv_fault: bool,              // short-circuited to VCC
	internal_temperature: u16,    // reference junction temperature (in degrees C * 4)
	fault: bool,                  // set if any fault is present
	thermocouple_temperature: u16 // thermocouple temperature (in degrees C * 4)
}

impl Thermocouple {
	//
	// open(path) - open thermocouple attached to the specified spidev file
	//
	pub fn open<P: AsRef<Path>>(path: P) -> Result<Thermocouple> {
		match File::open(path) {
			Ok(f) => Ok(Thermocouple { spi: f }),
			Err(_) => Err("unable to open thermocouple")
		}
	}

	//
	// read_sample() - read and return a sample from the thermocouple
	//
	pub fn read_sample(&mut self) -> Result<Sample> {
		let mut buf = [0u8; 4];
		match self.spi.read(&mut buf) {
			Err(_) => Err("unable to read from thermocouple"),
			Ok(bytes_read) =>
				if bytes_read != 4 {
					Err("no more samples")
				} else {
					Ok(Sample::new(
						(buf[0] as u32) << 24 |
						(buf[1] as u32) << 16 |
						(buf[2] as u32) <<  8 |
						(buf[3] as u32)))
				}
		}
	}
}

impl Sample {
	//
	// new(raw) - constructs a sample from the 32-bit value read from the thermocouple
	//
	pub fn new(raw: u32) -> Sample {
		Sample {
			oc_fault: (raw >> 0) & 0x01 == 0x01,
			scg_fault: (raw >> 1) & 0x01 == 0x01,
			scv_fault: (raw >> 2) & 0x01 == 0x01,
			internal_temperature: ((raw >> 3) & 0xFFF) as u16,
			fault: (raw >> 15) & 0x01 == 0x01,
			thermocouple_temperature: (raw >> 18) as u16,
		}
	}

	//
	// get_temp_celcius() - calculate and return the temperature in celcius
	//
	pub fn get_temp_celcius(&self) -> Option<f32> {
		if self.fault {
			None
		} else {
			Some(self.thermocouple_temperature as f32 / 4f32)
		}
	}

	//
	// get_temp_fahrenheit() - calculate and return the temperature in fahrenheit
	//
	pub fn get_temp_fahrenheit(&self) -> Option<f32> {
		if self.fault {
			None
		} else {
			Some(self.thermocouple_temperature as f32 * 0.45f32 + 32f32)
		}
	}
}

impl fmt::Display for Sample {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.fault {
			if self.oc_fault {
				write!(f, "fault (open connection)")
			} else if self.scg_fault {
				write!(f, "fault (shorted to GND)")
			} else if self.scv_fault {
				write!(f, "fault (shorted to VCC)")
			} else {
				write!(f, "fault (unknown)")
			}
		} else {
			write!(f, "{} (F)", self.get_temp_fahrenheit().unwrap())
		}
	}
}
