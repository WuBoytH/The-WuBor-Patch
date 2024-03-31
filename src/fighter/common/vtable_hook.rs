mod brave;
mod dolly;
mod eflame;
mod elight;
mod ganon;
mod gaogaen;
mod ike;
mod jack;
mod koopa;

mod weapon;

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

    weapon::install();
}