mod special_n_jump;
pub mod special_s;
pub mod special_lw;
mod summon;
mod dispatch;
mod doyle;
pub mod helper;

pub fn install() {
    special_n_jump::install();
    special_s::install();
    summon::install();
    dispatch::install();
    doyle::install();
}