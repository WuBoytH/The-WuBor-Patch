mod brave;

mod weapon;

pub fn install() {
    brave::install();

    weapon::install();
}