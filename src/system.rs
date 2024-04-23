mod init_settings;
mod get_val;
mod sound;
pub mod func_links;
mod fighterutil;
mod menu;
mod music;
mod fighterspecializer;
mod engine;
pub mod controller;
mod modules;
mod one_frame;
mod css;

pub fn install() {
    init_settings::install();
    get_val::install();
    sound::install();
    fighterutil::install();
    menu::install();
    music::install();
    fighterspecializer::install();
    engine::install();
    controller::install();
    modules::install();
    one_frame::install();
    css::install();
}