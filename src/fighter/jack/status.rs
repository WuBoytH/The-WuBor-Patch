pub mod special_s;
pub mod special_lw;
mod summon;
mod dispatch;
mod doyle;

pub fn install() {
    special_s::install();
    summon::install();
    dispatch::install();
    doyle::install();
}