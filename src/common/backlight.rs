use log::error;
use std::process::Command;

/*
 * TODOs:
 * - add programmatic backlight control instead of using shell command.
 */

fn xbacklight(dir: &str, amount: &str) -> u8 {
    match Command::new("xbacklight").args([dir, amount]).output() {
        Ok(_) => 0,
        Err(e) => {
            error!("xbacklight {}, {}, error: {}", dir, amount, e);
            4
        }
    }
}

pub fn inc_bright(amount: &str) -> u8 {
    xbacklight("-inc", amount)
}

pub fn dec_bright(amount: &str) -> u8 {
    xbacklight("-dec", amount)
}
