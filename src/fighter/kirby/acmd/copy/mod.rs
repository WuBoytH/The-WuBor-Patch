mod ganon;
mod ike;
mod belmont;
mod jack;

pub fn install() {
    ganon::install();
    ike::install();
    belmont::install();
    jack::install();
}