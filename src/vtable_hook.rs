mod fighter;
mod dolly;
mod eflame;
mod jack;
mod kirby;
mod lucario;
// mod ness;
mod rockman;
mod shotos;
mod shulk;
mod wario;

mod weapon;
mod blaster_bullet;
// mod ike_sword;
mod inkling_rollerink;

pub fn install() {
    fighter::install();
    dolly::install();
    eflame::install();
    jack::install();
    kirby::install();
    lucario::install();
    // ness::install();
    rockman::install();
    shotos::install();
    shulk::install();
    wario::install();

    weapon::install();
    blaster_bullet::install();
    // ike_sword::install();
    inkling_rollerink::install();
}
