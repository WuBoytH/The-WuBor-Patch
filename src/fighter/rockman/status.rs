mod attack;
mod attack_s3;
mod attack_air;
mod ladder_attack;
mod special_n;
mod rockbuster;
mod special_lw;
pub mod helper;

pub fn install() {
    attack::install();
    attack_s3::install();
    attack_air::install();
    ladder_attack::install();
    special_n::install();
    rockbuster::install();
    special_lw::install();
}
