use super::*;

mod attack;
mod escape;
mod appeal;

pub fn install() {
    attack::install();
    escape::install();
    appeal::install();
}