use {
    smash::{
        lua2cpp::*,
        phx::Hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*,
    custom_var::*,
    wubor_utils::vars::*
};

#[status_script(agent = "bayonetta", status = FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn bayonetta_specialairs_d_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_BAYONETTA_SPECIAL_AIR_S);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_s_d"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        bayonetta_specialairs_d_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(bayonetta_specialairs_d_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_specialairs_d_main_loop as *const () as _))
}

unsafe extern "C" fn bayonetta_specialairs_d_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool()
    && VarModule::is_flag(fighter.battle_object, bayonetta::status::flag::SPECIAL_AIR_S_D_IS_BOUNCE)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_D_FLAG_HIT) {
        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT.into(), false.into());
        fighter.global_table[SUB_STATUS2].assign(&L2CValue::I32(0));
        fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
    }
    0.into()
}

unsafe extern "C" fn bayonetta_specialairs_d_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    // if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_D_FLAG_HIT) {
    //     if MotionModule::is_end(fighter.module_accessor) {
    //         if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
    //             fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    //             return 0.into();
    //         }
    //     }
    // }
    // else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK) {
            let flag = if PostureModule::lr(fighter.module_accessor) < 0.0 {
                *GROUND_TOUCH_FLAG_LEFT as u32
            }
            else {
                *GROUND_TOUCH_FLAG_RIGHT as u32
            };
            if GroundModule::is_touch(fighter.module_accessor, flag) {
                fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END.into(), false.into());
            }
        }
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if MotionModule::is_end(fighter.module_accessor) {
                if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                    return 0.into();
                }
            }
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_LANDING_FALL_SPECIAL)
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_LANDING.into(), false.into());
            }
        }
    // }
    0.into()
}

#[status_script(agent = "bayonetta", status = FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn bayonetta_specialairs_d_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("special_air_s_d_landing"));
    let rate = end_frame / 40.0;
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_s_d_landing"),
        0.0,
        rate,
        false,
        0.0,
        false,
        false
    );
    bayonetta_reset_landing(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_specialairs_d_landing_main_loop as *const () as _))
}

unsafe extern "C" fn bayonetta_specialairs_d_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn bayonetta_reset_landing(fighter: &mut L2CFighterCommon) {
    bayonetta_reset_abk(fighter);
    bayonetta_reset_witchtwist(fighter);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME);
}

unsafe extern "C" fn bayonetta_reset_abk(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_REUSE_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
}

unsafe extern "C" fn bayonetta_reset_witchtwist(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AFTER_ACTION);
}

pub fn install() {
    install_status_scripts!(
        bayonetta_specialairs_d_main,
        bayonetta_specialairs_d_landing_main
    );
}