mod kirby;
mod ganon;
mod lucario;
mod ike;
mod belmont;

pub fn install() {
    kirby::install();
    ganon::install();
    lucario::install();
    ike::install();
    belmont::install();
}