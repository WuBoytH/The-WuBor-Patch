use super::*;

mod look_up;
mod look_up_wait;
mod look_up_rv;

mod turn;

pub mod dash;

mod run;
mod run_brake;
mod turn_run;

mod jump_squat;
mod jump;

mod landing;

mod pass;

mod tread_jump;

pub fn install() {
    turn::install();

    dash::install();

    run::install();
    run_brake::install();
    turn_run::install();

    jump_squat::install();
    jump::install();

    landing::install();

    pass::install();

    tread_jump::install();
}