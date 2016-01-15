#![feature(time2)] 

#[macro_use] extern crate nickel;
#[macro_use] extern crate lazy_static;
extern crate rustc_serialize;

pub mod hw;
pub mod sousvide;

use std::f32;
use std::sync::{Mutex};
use std::thread::{spawn, sleep};

use nickel::{Nickel, HttpRouter, StaticFilesHandler, JsonBody};
use rustc_serialize::json;
use sousvide::SousVide;

const THERMOCOUPLE_SPI_DEV: &'static str = "/dev/spidev0.0";
const HEATER_PIN: u8 = 17;
const PUMP_PIN: u8 = 27;

#[derive(RustcEncodable)]
struct State {
	heater : bool,
	pump : bool,
	cur_temp : f32,
	set_temp : f32
}

#[derive(RustcDecodable)]
struct SetTempBody {
	value : f32
}

lazy_static! {
	static ref SOUSVIDE : Mutex<SousVide> = Mutex::new(SousVide::new(THERMOCOUPLE_SPI_DEV, PUMP_PIN, HEATER_PIN));
}

fn main() {
	spawn(move || {
		loop {
			let delay = SOUSVIDE.lock().unwrap().step();
			if delay.is_some() {
				sleep(delay.unwrap());
			}
		}
	});

	let mut server = Nickel::new();
	server.utilize(StaticFilesHandler::new("public/"));
	server.get("/rest/state", middleware!(|_req| {
		let sv = SOUSVIDE.lock().unwrap();
		json::encode(
			& State {
				heater: sv.get_heater_state(),
				pump: sv.get_pump_state(),
				cur_temp: sv.get_cur_temp().unwrap_or(f32::NAN),
				set_temp: sv.get_set_temp().unwrap_or(f32::NAN)
			}).unwrap()
	}));
	server.get("/rest/state/heater",   middleware!(|_| { json::encode(&SOUSVIDE.lock().unwrap().get_heater_state()).unwrap()}));
	server.get("/rest/state/pump",     middleware!(|_| { json::encode(&SOUSVIDE.lock().unwrap().get_pump_state()).unwrap()}));
	server.get("/rest/state/cur_temp", middleware!(|_| { json::encode(&SOUSVIDE.lock().unwrap().get_cur_temp().unwrap_or(f32::NAN)).unwrap()}));
	server.get("/rest/state/set_temp", middleware!(|_| { json::encode(&SOUSVIDE.lock().unwrap().get_set_temp().unwrap_or(f32::NAN)).unwrap()}));
	server.put("/rest/state/set_temp", middleware!(|req| {
		let body = req.json_as::<SetTempBody>();
		if body.is_ok() {
			SOUSVIDE.lock().unwrap().change_set_temp(body.unwrap().value);
		} else {
			SOUSVIDE.lock().unwrap().clear_set_temp();
		}
	}));
	server.listen("0.0.0.0:8080");
}
