mod attack;
mod attack_s4;
mod special_hi_rush;
mod special_hi_rush_end;

pub fn install() {
    attack::install();
    attack_s4::install();
    special_hi_rush::install();
    special_hi_rush_end::install();
}