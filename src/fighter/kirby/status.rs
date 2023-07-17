mod kirby;
mod purin;
mod ganon;
mod lucario;
mod ike;
mod belmont;
mod jack;

pub fn install() {
    kirby::install();
    purin::install();
    ganon::install();
    lucario::install();
    ike::install();
    belmont::install();
    jack::install();
}