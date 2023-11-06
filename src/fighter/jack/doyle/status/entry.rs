use crate::imports::status_imports::*;
use super::helper::*;

pub unsafe extern "C" fn jack_doyle_entry_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = LinkModule::get_parent_id(weapon.module_accessor, *LINK_NO_CONSTRAINT, true) as u32;
    let owner = sv_battle_object::module_accessor(owner_id);
    let rebel_gauge = WorkModule::get_float(owner, 0x4D);
    WorkModule::mul_float(weapon.module_accessor, rebel_gauge / 100.0, 0x6);
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("entry"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    weapon.fastshift(L2CValue::Ptr(jack_doyle_entry_main_loop as *const () as _))
}

unsafe extern "C" fn jack_doyle_entry_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if MotionModule::is_end(weapon.module_accessor) {
        jack_doyle_set_flags(weapon);
        weapon.change_status(WEAPON_JACK_DOYLE_STATUS_KIND_FOLLOW.into(), false.into());
    }
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *WEAPON_JACK_DOYLE_STATUS_KIND_ENTRY, jack_doyle_entry_main);
}