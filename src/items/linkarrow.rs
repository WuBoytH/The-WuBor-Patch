use crate::imports::*;
use crate::system::func_links;

pub static mut LINK_ARROW_THROW : usize = 0x6ca62c;

unsafe extern "C" fn link_arrow_throw(ctx: &mut skyline::hooks::InlineCtx) {
    let item: &mut L2CAgent = std::mem::transmute(ctx.registers[20].x.as_mut());

    let mut speed_x = KineticModule::get_sum_speed_x(item.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut speed_y = KineticModule::get_sum_speed_y(item.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    let angle = func_links::LINKARROW::ITEM_THROW_DEGREE();
    let speed_length = sv_math::vec2_length(speed_x, speed_y);
    let rad = angle.to_radians();
    speed_x = speed_length.abs() * rad.cos() * speed_x.signum();
    speed_y = speed_length.abs() * rad.sin() * speed_y.signum();

    func_links::kinetic_energy_outer::set_speed_2f(item.lua_state_agent, &Vector2f{x: speed_x, y: 0.0});
    func_links::kinetic_energy_gravity::set_speed_1f(item.lua_state_agent, speed_y);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "item" {
        unsafe {
            let base = (*info.module.ModuleObject).module_base as usize;
            LINK_ARROW_THROW += base;

            skyline::hooks::A64InlineHook(
                LINK_ARROW_THROW as u64 as _,
                link_arrow_throw as _
            );
            let _ = skyline::patching::patch_pointer(LINK_ARROW_THROW as *const u8, &0x14000054u32);
        }
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}