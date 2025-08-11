mod inkling;
mod jack;
mod brave;
mod pickel;

pub fn install() {
    inkling::install();
    jack::install();
    brave::install();
    pickel::install();
}