use super::*;

unsafe extern "C" fn set_bonker(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let module_accessor = sv_battle_object::module_accessor(owner);
    
    let is_bonker = VarModule::is_flag(module_accessor, vars::mario::instance::flag::BONKER);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("hammer"), !is_bonker);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("bonker"), is_bonker);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, 0, set_bonker);
}