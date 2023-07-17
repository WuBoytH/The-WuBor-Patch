pub mod breath;
mod special_hi_charge;

pub fn install() {
    breath::install();
    special_hi_charge::install();
}