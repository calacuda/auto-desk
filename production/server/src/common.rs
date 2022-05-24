// use std::collections::HashMap;
// use std::error::Error;
// use std::io::prelude::*;
use rdev::{simulate, EventType, Key, SimulateError};
use shellexpand;
use std::process::Command;
use std::{thread, time};

pub mod backlight;
pub mod media;
pub mod power;
pub mod xrandr;

/*
 * TODOs:
 * - make the open_program routine like that of open_on_desktop form the BSPWM mod
 *
 */

pub fn get_layout_file(file_name: &str) -> String {
    return shellexpand::tilde(&if file_name.ends_with(".layout") {
        format!("~/.config/desktop-automater/layouts/{}", file_name)
    } else {
        format!("~/.config/desktop-automater/layouts/{}.layout", file_name)
    })
    .to_string();
}

pub fn open_program(program: &str) -> u8 {
    println!("[LOG] running: {}", program);
    let _process = Command::new(program)
        .spawn()
        .expect("failed to execute process");
    println!("[LOG] ran {}", program);
    return 0;
}

fn send_key_stroke(event_type: &EventType) -> u8 {
    let delay = time::Duration::from_millis(20);
    let res = match simulate(event_type) {
        Ok(()) => 0,
        Err(SimulateError) => {
            println!("[ERROR] print screen key not pressed");
            4
        }
    };
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
    return res;
}

pub fn screen_shot() -> u8 {
    let press = send_key_stroke(&EventType::KeyPress(Key::PrintScreen));
    let release = send_key_stroke(&EventType::KeyRelease(Key::PrintScreen));

    return if press > 0 {
        press
    } else if release > 0 {
        release
    } else {
        0
    };
}
