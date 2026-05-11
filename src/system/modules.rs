mod control;
mod grab;

pub fn install() {
    control::install();
    grab::install();
}
