use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        table_const::*
    }
};

#[status_script(agent = "daisy", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn daisy_attackair_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_AttackAir()
}

#[status_script(agent = "daisy", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn daisy_attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(daisy_attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn daisy_attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackAir_Main()
}

#[status_script(agent = "daisy", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn daisy_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_landing_mot_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_AIR_HI_START);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_ENABLE_UNIQ);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    // let special_hi_parasol_limit_time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_parasol_limit_time"));
    // WorkModule::set_int(fighter.module_accessor, special_hi_parasol_limit_time, *FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_PARASOL_LIMIT_TIME_COUNTER);
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_PEACH_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    fighter.sub_shift_status_main(L2CValue::Ptr(daisy_specialhi_main_loop as *const () as _))
}

unsafe extern "C" fn daisy_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() == false {
        // let enable_uniq = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_ENABLE_UNIQ);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_HI_FLAG_MOVE_TRANS) {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_HI_MOTION_AIR_ANGLE);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_ENABLE_UNIQ);
        }
        if MotionModule::is_end(fighter.module_accessor) {
            // fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
            fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "daisy", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn daisy_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

#[status_script(agent = "daisy", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn daisy_speciallw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        FighterSpecializer_Peach::special_lw_check_num_of_item(fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor);
        WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_NONE as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND);
    }
    0.into()
}

#[status_script(agent = "daisy", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn daisy_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_ITEM_NO_COUNT);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.clear_lua_stack();
        lua_args!(
            fighter,
            0x20cbc92683u64,
            1,
            *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND,
            *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_04 - 1
        );
        fighter.pop_lua_stack(1);
        fighter.sub_shift_status_main(L2CValue::Ptr(daisy_speciallw_main_loop as *const () as _))
    }
    else {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START.into(), false.into());
        return 1.into();
    }
}

unsafe extern "C" fn daisy_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        daisy_speciallwground_main_loop_helper(fighter);
    }
    else {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
        && fighter.sub_air_check_fall_common().get_bool() == false {
            daisy_speciallwground_main_loop_helper(fighter);
        }
    }
    0.into()
}

unsafe extern "C" fn daisy_speciallwground_main_loop_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
}

#[status_script(agent = "daisy", status = FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn daisy_uniqfloatstart_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    0.into()
}

#[status_script(agent = "daisy", status = FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn daisy_uniqfloatstart_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 18.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("fuwafuwa_start"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(daisy_uniqfloatstart_main_loop as *const () as _))
}

unsafe extern "C" fn daisy_uniqfloatstart_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() == false {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "daisy", status = FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn daisy_uniqfloatstart_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR) {
            ArticleModule::remove(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    0.into()
}

#[status_script(agent = "daisy", status = FIGHTER_STATUS_KIND_FALL_SPECIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn daisy_fallspecial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall_special"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(daisy_fallspecial_main_2 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(daisy_fallspecial_main_loop as *const () as _))
}

unsafe extern "C" fn daisy_fallspecial_main_2(fighter: &mut L2CFighterCommon, param2 : L2CValue) -> L2CValue {
    fighter.sub_fall_common_uniq(param2)
}

unsafe extern "C" fn daisy_fallspecial_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() == false {
        if fighter.sub_fall().get_bool() == false {
            // let parasol_timer = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_PARASOL_LIMIT_TIME_COUNTER);
            // if !(0 < parasol_timer) {
                return daisy_fallspecial_main_loop_helper(fighter);
            // }
            // let stick_y = fighter.global_table[STICK_Y].get_f32();
            // let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
            // let special_hi_float;
            // if jump_stick_y > stick_y {
            //     special_hi_float = false;
            // }
            // else {
            //     special_hi_float = ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR);
            // }
            // if !special_hi_float {
            //     return daisy_fallspecial_main_loop_helper(fighter);
            // }
            // fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_FALL.into(), true.into());
            // return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn daisy_fallspecial_main_loop_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

// #[status_script(agent = "daisy", status = FIGHTER_STATUS_KIND_ITEM_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn daisy_itemthrow_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let lr = PostureModule::lr(fighter.module_accessor);
//     WorkModule::set_float(fighter.module_accessor, lr, *FIGHTER_STATUS_ITEM_THROW_WORK_FLOAT_LR);
//     let mut throw_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
//     if throw_motion == 0 {
//         if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
//             fighter.ItemThrowLightMotionDecisionAir();
//         }
//         else {
//             fighter.ItemThrowLightMotionDecision();
//         }
//     }
//     throw_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
//     if ![
//         0x13455c51bc,
//         0x13213094b8,
//         0x1434afb6dd,
//         0x14e7e721de,
//         0x13ce05517b,
//         0x135066a91c,
//         0x1579bbe8d8,
//         0x128c67b93f,
//         0x128b0a7d26,
//         0x176265cf32,
//         0x1706090a36,
//         0x183e882164,
//         0x18edc0b667,
//         0x16b7b89d4e,
//         0x16b0d55957,
//         0x17e93ccff5,
//         0x17775f3792
//     ].contains(&throw_motion) {
//         if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
//             WorkModule::set_int64(fighter.module_accessor, 0x16b7b89d4e, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
//             WorkModule::set_int64(fighter.module_accessor, 0x128c67b93f, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND_OPPOSITE);
//         }
//         else {
//             WorkModule::set_int64(fighter.module_accessor, 0x128c67b93f, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
//             WorkModule::set_int64(fighter.module_accessor, 0x16b7b89d4e, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND_OPPOSITE);
//         }
//     }
//     ControlModule::clear_command(fighter.module_accessor, false);
//     ControlModule::reset_flick_x(fighter.module_accessor);
//     ControlModule::reset_flick_sub_x(fighter.module_accessor);
//     fighter.global_table[FLICK_X].assign(&0xfe.into());
//     ControlModule::reset_trigger(fighter.module_accessor);
//     WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_NONE, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_SITUATION);
//     WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ITEM_THROW_WORK_FLAG_LOOP_FIRST);
//     fighter.status_ItemThrow_Loop();
//     fighter.sub_shift_status_main(L2CValue::Ptr(daisy_itemthrow_main_loop as *const () as _))
// }

// unsafe extern "C" fn daisy_itemthrow_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.status_ItemThrow_Main()
// }

#[status_script(agent = "daisy", status = FIGHTER_STATUS_KIND_ITEM_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn daisy_itemthrow_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    SPECIAL_THROW[entry_id(fighter.module_accessor)] = false;
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ITEM_THROW_4);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        daisy_attackair_pre,
        daisy_attackair_main,
        daisy_specialhi_main,
        daisy_speciallw_pre,
        daisy_speciallw_init,
        daisy_speciallw_main,
        daisy_uniqfloatstart_pre,
        daisy_uniqfloatstart_main,
        daisy_uniqfloatstart_end,
        daisy_fallspecial_main,
        // daisy_itemthrow_main,
        daisy_itemthrow_end
    );
}