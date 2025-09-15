use crate::imports::*;

unsafe extern "C" fn poisonbreath_on_hit(vtable: u64, weapon: *mut app::Weapon, something: u32) -> u64 {
    let module_accessor = (*weapon).battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    if status == *WEAPON_PACKUN_POISONBREATH_STATUS_KIND_START
    && something & 6 != 0 {
        StatusModule::change_status_request((*weapon).battle_object.module_accessor, packun_poisonbreath::status::BURST, false);
        return 1;
    }
    MiscModule::normal_weapon_hit_handler(vtable, weapon, something)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x51f5f90).data(poisonbreath_on_hit as u64);
}