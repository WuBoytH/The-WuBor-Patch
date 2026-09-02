use super::*;

unsafe extern "C" fn ganon_special_air_s_catch_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_CATCH_SET_CATCH);
    sv_module_access::_catch(fighter.lua_state_agent);
    // let func = smashline::api::get_target_function("lua2cpp_ganon.nrs", 0x6ef0).unwrap();
    // let func : fn(&mut L2CFighterCommon, L2CValue, L2CValue) = std::mem::transmute(func);
    // func(fighter, hash40("catched_ganon").into(), hash40("catched_air_ganon").into());
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    0.into()
}

unsafe extern "C" fn ganon_special_air_s_catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::set_rate(fighter.module_accessor, 0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_special_air_s_catch_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_special_air_s_catch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut ret = 0;
    if fighter.global_table[STATUS_FRAME].get_f32() == 1.0 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_catch"), 1.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

        let speed_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_X);
        let speed_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_Y);
        KineticModule::clear_speed_attr(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x,
            0.0
        );
    }
    if fighter.global_table[STATUS_FRAME].get_f32() >= 1.0 {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END.into(), false.into());
        }
        ret = 1;
    }
    ret.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, ganon_special_air_s_catch_init);
    agent.status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, ganon_special_air_s_catch_main);
}