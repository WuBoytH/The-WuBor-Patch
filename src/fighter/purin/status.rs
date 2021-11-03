use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::table_const::*
};

#[status_script(agent = "purin", status = FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HIT_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn purin_specialnhitend(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_hit_end"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_PURIN_STATUS_SPECIAL_N_WORK_FLOAT_MOTION_RATE);
    fighter.sub_shift_status_main(L2CValue::Ptr(purin_specialnhitendmain as *const () as _))
}

unsafe extern "C" fn purin_specialnhitendmain(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_air_check_fall_common();
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() == false {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        purin_specialnhitend
    );
}