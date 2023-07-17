mod catch;
mod capture_pulled;
mod catched_air_ganon;

pub fn install() {
    catch::install();
    capture_pulled::install();
    catched_air_ganon::install();
}