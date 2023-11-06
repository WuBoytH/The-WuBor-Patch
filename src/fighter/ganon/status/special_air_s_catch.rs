use crate::imports::status_imports::*;

unsafe extern "C" fn ganon_special_air_s_catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    MotionModule::set_rate(fighter.module_accessor, 0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_special_air_s_catch_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_special_air_s_catch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut ret = 0;
    if fighter.global_table[STATUS_FRAME].get_f32() == 1.0 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_catch"), 1.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    if fighter.global_table[STATUS_FRAME].get_f32() >= 1.0 {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END.into(), false.into());
        }
        ret = 1;
    }
    ret.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, ganon_special_air_s_catch_main);
}