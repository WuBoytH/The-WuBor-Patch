use crate::imports::*;

#[skyline::hook(offset = 0x33ecf60)]
unsafe extern "C" fn flare1_init(vtable: u64, weapon: *mut app::Weapon, something: u64) {
    let module_accessor = (*weapon).battle_object.module_accessor;

    let mut power = *(something as *mut f32).add(0x88 / 0x4);
    let mut ratio = *(something as *mut f32).add(0x8c / 0x4);
    // let angle = *(something as *mut f32).add(0x90 / 0x4);
    // println!("Power: {}, Ratio: {}, Angle: {}", power, ratio, angle);

    if ratio > 1.0 {
        ratio = 0.0;
        power = 1.0;
        VarModule::on_flag(module_accessor, edge_flare1::instance::flag::IS_GUARD_CANCEL);
    }

    *(something as *mut f32).add(0x88 / 0x4) = power;
    *(something as *mut f32).add(0x8c / 0x4) = ratio;

    original!()(vtable, weapon, something)
}

pub fn install() {
    skyline::install_hooks!(
        flare1_init
    );
}