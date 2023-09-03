mod turn;
pub mod dash;
mod run;
mod run_brake;
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
    jump_squat::install();
    jump::install();
    landing::install();
    pass::install();
    tread_jump::install();
}