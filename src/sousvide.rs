//
// imports
//
use hw::max31855::Thermocouple;
use hw::gpio::{Pin, Direction, State};
use std::time::{Instant, Duration};
use std::thread::sleep;

//
// constants
//
const HEAT_THRESHOLD: f32 = 0.5; // start heating when we fall this far below the temperature
const COOL_THRESHOLD: f32 = 0.0; // start cooling when we reach this far above the temperature
const STEP_PERIOD: u32 = 1000000000; // desired step period in nanoseconds
const THERMOCOUPLE_CORRECTION: f32 = -3.0; // correction constant for the thermocouple

//
// SousVide structure
//
// Holds the state of the sousvide.
//
pub struct SousVide {
	thermocouple: Thermocouple, // the thermocouple in use
	pump: Pin,                  // the pin that controls the pump
	heater: Pin,                // the pin that contorls the heater
	set_temp: Option<f32>,      // the desired temperature
	cur_temp: Option<f32>,      // the current temperature
	pump_state: bool,           // the current state of the pump
	heater_state: bool,         // the current state of the heater
}

impl SousVide {
	//
	// constructor for new sousvides
	//
	pub fn new(tc_dev: &str, pump_pin: u8, heater_pin: u8) -> SousVide {
		let mut sv = SousVide {
			thermocouple: Thermocouple::open(tc_dev).unwrap(),
			pump: Pin::open(pump_pin, Direction::Out).unwrap(),
			heater: Pin::open(heater_pin, Direction::Out).unwrap(),
			set_temp: None,
			cur_temp: None,
			pump_state: false,
			heater_state: false,
		};
		// default the pump and heater to off
		sv.set_pump_state(false);
		sv.set_heater_state(false);
		sv
	}

	//
	// clear_set_temp() - clears the set_temp, therby disabling the sousvide
	//
	pub fn clear_set_temp(&mut self) {
		println!("set_temp: Cleared");
		self.set_temp = None;
	}

	//
	// change_set_temp(f32) - changes the set_temp
	//
	pub fn change_set_temp(&mut self, set_temp: f32) {
		println!("set_temp: {}", set_temp);
		self.set_temp = Some(set_temp)
	}

	//
	// getters for current state
	//
	pub fn get_set_temp(&self) -> Option<f32> { self.set_temp }
	pub fn get_cur_temp(&self) -> Option<f32> { self.cur_temp }
	pub fn get_pump_state(&self) -> bool { self.pump_state }
	pub fn get_heater_state(&self) -> bool { self.heater_state }

	//
	// setup() - performs one-time setup
	//
	pub fn setup(&mut self) {
		// wait for thermocoupler chip to stabilize
		sleep(Duration::from_millis(500)); 
	}

	//
	// step() - updates the state of the sousvide and returns time until next step should occur
	//
	pub fn step(&mut self) -> Option<Duration> {
		let step_start = Instant::now();	

		// get the current temperature
		self.cur_temp = self.thermocouple.read_sample().ok().map(|t| { t.get_temp_fahrenheit().unwrap() + THERMOCOUPLE_CORRECTION });

		if self.cur_temp.is_some() {
			println!("cur_temp: {}", self.cur_temp.unwrap());
		} else {
			println!("cur_temp: Unknown");
		}

		// if the temp can't be read, or the set_temp isn't set
		if self.set_temp.is_none() || self.cur_temp.is_none() {
			// make sure the pump and heater are off
			self.set_pump_state(false);
			self.set_heater_state(false);
		// otherwise
		} else {
			// make sure the pump is on
			self.set_pump_state(true);

			// if we are too cold, start heating
			if self.set_temp.unwrap() - self.cur_temp.unwrap() > HEAT_THRESHOLD {
				self.set_heater_state(true);
			}

			// if we are too hot, stop heating
			if self.cur_temp.unwrap() - self.set_temp.unwrap() > COOL_THRESHOLD {
				self.set_heater_state(false);
			}
		}

		// calculate and return delay before next step
		let step_end = Instant::now();
		let step_duration = step_end.duration_from_earlier(step_start);
		if step_duration.as_secs() == 0 && step_duration.subsec_nanos() < STEP_PERIOD {
			Some(Duration::new(0, STEP_PERIOD - step_duration.subsec_nanos()))
		} else {
			None
		}
	}

	//
	// set_heater_state(bool) - change the heater state, if necessary
	//
	fn set_heater_state(&mut self, state: bool) {
		if self.heater_state != state {
			println!("heater: {}", state);
			self.heater.set_state(if state { State::High } else { State::Low }).unwrap();
			self.heater_state = state;
		}
	}

	//
	// set_pump_state(bool) - change the pump state, if necessary
	//
	fn set_pump_state(&mut self, state: bool) {
		if self.pump_state != state {
			println!("pump: {}", state);
			self.pump.set_state(if state { State::High } else { State::Low }).unwrap();
			self.pump_state = state;
		}
	}
}
