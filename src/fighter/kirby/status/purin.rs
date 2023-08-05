use crate::imports::status_imports::*;

#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_PURIN_SPECIAL_N_HIT_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn kirby_purin_specialn_hit_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    FighterMotionModuleImpl::change_motion_kirby_copy(
        fighter.module_accessor,
        Hash40::new("special_air_n_hit_end"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_PURIN_STATUS_SPECIAL_N_WORK_FLOAT_MOTION_RATE);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_purin_specialn_hit_end_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_purin_specialn_hit_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !fighter.sub_transition_group_check_air_cliff().get_bool()
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        kirby_purin_specialn_hit_end_main
    );
}