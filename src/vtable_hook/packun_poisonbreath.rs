use crate::imports::*;

unsafe extern "C" fn poisonbreath_on_hit(vtable: u64, weapon: *mut app::Weapon, something: u32) -> u64 {
    let module_accessor = (*weapon).battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    if status == *WEAPON_PACKUN_POISONBREATH_STATUS_KIND_START
    && something & 6 != 0 {
        StatusModule::change_status_request((*weapon).battle_object.module_accessor, packun_poisonbreath::status::BURST, false);
        return 1;
    }
    normal_weapon_hit_handler(vtable, weapon, something)
}

#[skyline::from_offset(0x33bdc30)]
unsafe extern "C" fn normal_weapon_hit_handler(vtable: u64, weapon: *mut app::Weapon, something: u32) -> u64;

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x51f6f90).data(poisonbreath_on_hit as u64);
}