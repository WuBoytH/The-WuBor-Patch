use super::*;

unsafe extern "C" fn ganon_special_air_s_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        1
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        true,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

// unsafe extern "C" fn ganon_special_air_s_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let func = smashline::api::get_target_function("lua2cpp_ganon.nrs", 0x6ef0).unwrap();
//     let func : fn(&mut L2CFighterCommon, L2CValue, L2CValue) = std::mem::transmute(func);
//     func(fighter, hash40("throw").into(), hash40("invalid").into());
//     0.into()
// }

unsafe extern "C" fn ganon_special_air_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.4
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_special_air_s_end_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_special_air_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_air_check_fall_common().get_bool() {
        return 1.into()
    }
    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    1.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END, ganon_special_air_s_end_pre);
    // agent.status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END, ganon_special_air_s_end_init);
    agent.status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END, ganon_special_air_s_end_main);
}