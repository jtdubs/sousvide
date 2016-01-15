use hw::max31855::Thermocouple;
use hw::gpio::{Pin, Direction, State};
use std::time::{Instant, Duration};
use std::thread::sleep;

const HEAT_THRESHOLD: f32 = 0.5;
const COOL_THRESHOLD: f32 = 0.0;

const LOOP_INTERVAL: u32 = 1000000000;

pub struct SousVide {
	thermocouple: Thermocouple,
	pump: Pin,
	heater: Pin,
	set_temp: Option<f32>,
	cur_temp: Option<f32>,
	pump_state: bool,
	heater_state: bool,
}

impl SousVide {
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
		sv.set_pump_state(false);
		sv.set_heater_state(false);
		sv
	}

	pub fn clear_set_temp(&mut self) {
		println!("Set Temp: Cleared");
		self.set_temp = None;
	}

	pub fn change_set_temp(&mut self, set_temp: f32) {
		println!("Set Temp: {}", set_temp);
		self.set_temp = Some(set_temp)
	}

	pub fn get_set_temp(&self) -> Option<f32> { self.set_temp }
	pub fn get_cur_temp(&self) -> Option<f32> { self.cur_temp }
	pub fn get_pump_state(&self) -> bool { self.pump_state }
	pub fn get_heater_state(&self) -> bool { self.heater_state }

	pub fn setup(&mut self) {
		// wait for thermocoupler chip to stabilize
		sleep(Duration::from_millis(500)); 
	}

	pub fn step(&mut self) -> Option<Duration> {
		let step_start = Instant::now();	

		self.cur_temp = self.thermocouple.read_sample().ok().map(|t| { t.get_temp_fahrenheit().unwrap() - 3f32 });

		if self.cur_temp.is_some() {
			println!("Cur Temp: {}", self.cur_temp.unwrap());
		} else {
			println!("Cur Temp: Unknown");
		}


		if self.set_temp.is_none() || self.cur_temp.is_none() {
			self.set_pump_state(false);
			self.set_heater_state(false);
		} else {
			self.set_pump_state(true);

			if self.set_temp.unwrap() - self.cur_temp.unwrap() > HEAT_THRESHOLD {
				self.set_heater_state(true);
			}

			if self.cur_temp.unwrap() - self.set_temp.unwrap() > COOL_THRESHOLD {
				self.set_heater_state(false);
			}
		}

		let step_end = Instant::now();
		let step_duration = step_end.duration_from_earlier(step_start);
		if step_duration.as_secs() == 0 && step_duration.subsec_nanos() < LOOP_INTERVAL {
			Some(Duration::new(0, LOOP_INTERVAL - step_duration.subsec_nanos()))
		} else {
			None
		}
	}

	fn set_heater_state(&mut self, state: bool) {
		if self.heater_state != state {
			println!("heater: {}", state);
			self.heater.set_state(if state { State::High } else { State::Low }).unwrap();
			self.heater_state = state;
		}
	}

	fn set_pump_state(&mut self, state: bool) {
		if self.pump_state != state {
			println!("pump: {}", state);
			self.pump.set_state(if state { State::High } else { State::Low }).unwrap();
			self.pump_state = state;
		}
	}
}
