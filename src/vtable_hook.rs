mod fighter;
mod brave;
mod dolly;
mod eflame;
mod elight;
mod ganon;
mod gaogaen;
mod ike;
mod jack;
mod koopa;
mod lucario;
mod lucina;
mod luigi;
mod pickel;
mod reflet;
mod rockman;
mod shotos;
mod shulk;
mod sonic;
mod wario;

mod weapon;
mod belmont_cross;
mod dolly_wave;
mod mariod_drcapsule;
mod ryu_shinkuhadoken;

pub fn install() {
    fighter::install();
    brave::install();
    dolly::install();
    eflame::install();
    elight::install();
    ganon::install();
    gaogaen::install();
    ike::install();
    jack::install();
    koopa::install();
    lucario::install();
    lucina::install();
    luigi::install();
    pickel::install();
    reflet::install();
    rockman::install();
    shotos::install();
    shulk::install();
    sonic::install();
    wario::install();

    weapon::install();
    belmont_cross::install();
    dolly_wave::install();
    mariod_drcapsule::install();
    ryu_shinkuhadoken::install();
}