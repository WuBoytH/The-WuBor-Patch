mod attack_lw3;
mod special_lw;
mod holywater;

pub fn install() {
    attack_lw3::install();
    special_lw::install();
    holywater::install();
}