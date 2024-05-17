use super::*;

pub unsafe extern "C" fn brave_special_check_sp(
    fighter: &mut L2CFighterCommon,
    sp: L2CValue,
    success_flag: L2CValue
) {
    if brave_special_check_sp_set_flag(fighter, sp.clone(), success_flag).get_bool() {
        let fighta = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
        FighterSpecializer_Brave::add_sp(fighta, -sp.get_f32());
    }
}

unsafe extern "C" fn brave_special_check_sp_set_flag(
    fighter: &mut L2CFighterCommon,
    sp: L2CValue,
    success_flag: L2CValue
) -> L2CValue {
    let sp_current = WorkModule::get_float(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP);
    // let sp_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_MAX_SP);
    let ret = sp_current >= sp.get_f32();
    WorkModule::set_flag(fighter.module_accessor, ret, success_flag.get_i32());
    ret.into()
}
