use crate::imports::*;
use crate::system::func_links;

pub static mut GRENADE_STATUS_FALL : usize = 0x7c9ae0;
pub static mut GRENADE_STATUS_LANDING : usize = 0x7c9d10;
pub static mut GRENADE_STATUS_THROWN : usize = 0x7c9fc0;

#[skyline::hook(replace = GRENADE_STATUS_FALL)]
unsafe extern "C" fn snake_grenade_status_fall(item: &mut L2CAgent) -> L2CValue {
    TeamModule::set_hit_team(item.module_accessor, -1);
    HitModule::set_hit_stop_mul(item.module_accessor, 0.25, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_ALL as u8}, 0.0);
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_PICKUP);
    func_links::Item::disable_area(item.lua_state_agent, *ITEM_AREA_KIND_PICKUP);
    original!()(item)
}

#[skyline::hook(replace = GRENADE_STATUS_LANDING)]
unsafe extern "C" fn snake_grenade_status_landing(item: &mut L2CAgent) -> L2CValue {
    TeamModule::set_hit_team(item.module_accessor, -1);
    HitModule::set_hit_stop_mul(item.module_accessor, 0.25, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_ALL as u8}, 0.0);
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_PICKUP);
    func_links::Item::disable_area(item.lua_state_agent, *ITEM_AREA_KIND_PICKUP);
    original!()(item)
}

#[skyline::hook(replace = GRENADE_STATUS_THROWN)]
unsafe extern "C" fn snake_grenade_status_thrown(item: &mut L2CAgent) -> L2CValue {
    TeamModule::set_hit_team(item.module_accessor, -1);
    HitModule::set_hit_stop_mul(item.module_accessor, 0.25, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_ALL as u8}, 0.0);
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_PICKUP);
    func_links::Item::disable_area(item.lua_state_agent, *ITEM_AREA_KIND_PICKUP);
    original!()(item)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "item" {
        unsafe {
            // Makes it so hitting the grenade launches it instead of exploding
            let item_offset = (*info.module.ModuleObject).module_base as usize;
            skyline::patching::patch_pointer((0x7ca48c + item_offset) as *const u8, &0x529FE608u32);
            GRENADE_STATUS_FALL += item_offset;
            GRENADE_STATUS_LANDING += item_offset;
            GRENADE_STATUS_THROWN += item_offset;
            skyline::install_hooks!(
                snake_grenade_status_fall,
                snake_grenade_status_landing,
                snake_grenade_status_thrown
            );
        }
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}