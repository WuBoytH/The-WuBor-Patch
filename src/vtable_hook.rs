mod fighter;
mod brave;
mod dolly;
mod eflame;
mod elight;
mod gaogaen;
mod iceclimber;
mod jack;
mod kirby;
mod lucario;
// mod ness;
mod pickel;
mod reflet;
mod rockman;
mod shotos;
mod shulk;
mod wario;

mod weapon;
mod blaster_bullet;
// mod ike_sword;
mod inkling_rollerink;
mod mariod_drcapsule;

pub fn install() {
    fighter::install();
    brave::install();
    dolly::install();
    eflame::install();
    elight::install();
    gaogaen::install();
    iceclimber::install();
    jack::install();
    kirby::install();
    lucario::install();
    // ness::install();
    pickel::install();
    reflet::install();
    rockman::install();
    shotos::install();
    shulk::install();
    wario::install();

    weapon::install();
    blaster_bullet::install();
    // ike_sword::install();
    inkling_rollerink::install();
    mariod_drcapsule::install();
}
