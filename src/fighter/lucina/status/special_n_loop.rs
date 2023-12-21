use crate::imports::status_imports::*;

unsafe extern "C" fn lucina_special_n_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_n_loop"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_special_n_loop_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_special_n_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END.into(), false.into());
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            // if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END.into(), false.into());
            // }
            // else {
            //     // VarModule::on_flag(fighter.module_accessor, yu::instance::flag::HEROIC_GRAB);
            //     fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), false.into());
            // }
        }
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, lucina_special_n_loop_main);
}