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
mod wario;

mod weapon;
mod mariod_drcapsule;
mod belmont_cross;

pub fn install() {
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
    wario::install();

    weapon::install();
    mariod_drcapsule::install();
    belmont_cross::install();
}