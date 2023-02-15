mod ganon;
mod ike;
mod belmont;

pub fn install() {
    ganon::install();
    ike::install();
    belmont::install();
}