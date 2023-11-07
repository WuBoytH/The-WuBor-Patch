use crate::imports::status_imports::*;

unsafe extern "C" fn lucas_special_hi_reflect_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_DamageFlyReflect_effect(false.into());

    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_INT_NEXT_STATUS);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_hi_reflect"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if [
        *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK,
        *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD
    ].contains(&prev_status) {
        let accel_y_on_bound = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("accel_y_on_bound"));
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -accel_y_on_bound
        );
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(lucas_special_hi_reflect_main_loop as *const () as _))
}

unsafe extern "C" fn lucas_special_hi_reflect_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    let special_hi_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_INT_NUM);
    if special_hi_num != 0 && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
        fighter.change_status(FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_AGAIN.into(), true.into());
        return 1.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_REFLECT_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_DOWN.into(), false.into());
        }
        return 1.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_INT_NEXT_STATUS);
        fighter.change_status(status.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_REFLECT, lucas_special_hi_reflect_main);
}