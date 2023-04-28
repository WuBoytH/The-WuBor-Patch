mod walk;
mod attack;
mod attack_s3;
mod attack_s4;
mod attack_air;
mod ladder_attack;
mod airshooter;
mod special_n;
mod rockbuster;
mod chargeshot;
mod special_s;
mod special_lw;
mod rebirth;
pub mod helper;

pub fn install() {
    walk::install();
    attack::install();
    attack_s3::install();
    attack_s4::install();
    attack_air::install();
    ladder_attack::install();
    airshooter::install();
    special_n::install();
    rockbuster::install();
    chargeshot::install();
    special_s::install();
    special_lw::install();
    rebirth::install();
}
