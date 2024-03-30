use crate::imports::*;

unsafe extern "C" fn brave_special_lw_crash_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_RESET,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT as u32,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn brave_special_lw_crash_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        hash40("special_lw9_end")
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        hash40("special_air_lw9_end")
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(mot),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(brave_special_lw_crash_end_main_loop as *const () as _))
}

unsafe extern "C" fn brave_special_lw_crash_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_CRASH_END, brave_special_lw_crash_end_pre);
    agent.status(smashline::Main, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_CRASH_END, brave_special_lw_crash_end_main);
}