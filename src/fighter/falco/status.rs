mod attack_lw4;
mod special_hi_rush;
mod appeal;

pub fn install() {
    attack_lw4::install();
    special_hi_rush::install();
    appeal::install();
}