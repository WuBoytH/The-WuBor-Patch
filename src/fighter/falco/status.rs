mod attack;
mod attack_s4;
mod attack_lw4;
mod special_hi_rush;
mod appeal;

pub fn install() {
    attack::install();
    attack_s4::install();
    attack_lw4::install();
    special_hi_rush::install();
    appeal::install();
}