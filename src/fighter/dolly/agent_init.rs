use {
    smash::{
        lua2cpp::L2CFighterCommon,
        // phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    // smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*},
    super::{fgc::*, helper::*}
};

unsafe extern "C" fn dolly_guard_cont_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND)
    && dolly_check_special_hi_command(fighter).get_bool() {
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn dolly_check_ground_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, dolly::instance::flag::IS_SPECIAL_CANCEL);
    VarModule::off_flag(fighter.battle_object, dolly::instance::flag::ATTACK_DASH_COMMAND);
    false.into()
}

pub unsafe extern "C" fn dolly_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    if dolly_check_super_special_command(fighter).get_bool() {
        return true.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND)
    && dolly_check_special_hi_command(fighter).get_bool() {
        return true.into();
    }
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_LW_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND.into(), true.into());
        return true.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND) {
        VarModule::on_flag(fighter.battle_object, dolly::instance::flag::ATTACK_DASH_COMMAND);
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), true.into());
        return true.into();
    }
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND.into(), true.into());
        return true.into();
    }
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND.into(), true.into());
        return true.into();
    }
    // if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
    //     let cat2 = fighter.global_table[CMD_CAT2].get_i32();
    //     if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
    //     && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U) {
    //         fighter.clear_lua_stack();
    //         lua_args!(fighter, Hash40::new_raw(0x1daca540be));
    //         if fighter.pop_lua_stack(1).get_bool() {
    //             fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
    //             return true.into();
    //         }
    //     }
    //     if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0
    //     && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW) {
    //         fighter.clear_lua_stack();
    //         lua_args!(fighter, Hash40::new_raw(0x1daca540be));
    //         if fighter.pop_lua_stack(1).get_bool() {
    //             fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
    //             return true.into();
    //         }
    //     }
    //     if cat2 & (*FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0
    //     && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) {
    //         fighter.clear_lua_stack();
    //         lua_args!(fighter, Hash40::new_raw(0x1daca540be));
    //         if fighter.pop_lua_stack(1).get_bool() {
    //             fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
    //             return true.into();
    //         }
    //     }
    // }
    false.into()
}

unsafe extern "C" fn dolly_check_super_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    WorkModule::set_int(fighter.module_accessor, cat4, *FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_CAT4_SPECIAL_COMMAND);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2) {
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2.into(), true.into());
            return true.into();
        }
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL) {
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL.into(), true.into());
            return true.into();
        }
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2) {
            let opplr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
            if opplr != 0.0 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2.into(), true.into());
            return true.into();
        }
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL) {
            let opplr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
            if opplr != 0.0 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL.into(), true.into());
            return true.into();
        }
    }
    false.into()
}

unsafe extern "C" fn dolly_check_special_hi_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI2_COMMAND != 0
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_HI_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND.into(), true.into());
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn dolly_check_ground_catch_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && dolly_check_super_special_command(fighter).get_bool() {
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn dolly_status_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLAG_AUTO_TURN_END_STATUS);
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_STATUS_KIND_WAIT {
        FighterSpecializer_Dolly::update_opponent_lr_1on1(fighter.module_accessor, status);
    }
    let opponent_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if opponent_lr != 0.0 {
        if opponent_lr != PostureModule::lr(fighter.module_accessor) {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                let mut cont = false;
                if status != *FIGHTER_STATUS_KIND_WALK {
                    if [
                        *FIGHTER_STATUS_KIND_SQUAT,
                        *FIGHTER_STATUS_KIND_SQUAT_RV,
                        *FIGHTER_STATUS_KIND_LANDING,
                        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
                        *FIGHTER_STATUS_KIND_GUARD_ON,
                        *FIGHTER_STATUS_KIND_ESCAPE,
                        *FIGHTER_STATUS_KIND_ATTACK_HI3,
                        *FIGHTER_STATUS_KIND_ATTACK_LW3,
                        *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
                        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
                        *FIGHTER_STATUS_KIND_CATCH,
                        *FIGHTER_STATUS_KIND_ITEM_SWING,
                        *FIGHTER_STATUS_KIND_SPECIAL_N,
                        *FIGHTER_STATUS_KIND_FINAL,
                        *FIGHTER_RYU_STATUS_KIND_WALK_BACK,
                        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
                    ].contains(&status) {
                        cont = true;
                    }
                    else if !cont && status == *FIGHTER_STATUS_KIND_WAIT {
                        let status_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
                        if !dolly_status_end_control_lr_status_check(status_interrupt.into()).get_bool() {
                            cont = true;
                        }
                    }
                    else if !cont && status == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
                        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_STATUS_KIND_TURN_RUN {
                            cont = true;
                        }
                    }
                    else if !cont && status == *FIGHTER_STATUS_KIND_ATTACK {
                        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_ATTACK
                        && ComboModule::count(fighter.module_accessor) == 0 {
                            cont = true;
                        }
                    }
                    else if !cont && status == *FIGHTER_STATUS_KIND_ITEM_THROW {
                        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                            let cat3 = fighter.global_table[CMD_CAT3].get_i32();
                            if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_4 != 0 {
                                if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_FB4 == 0 {
                                    cont = true;
                                }
                            }
                            else {
                                if cat3 & (*FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI | *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_LW) != 0 {
                                    cont = true;
                                }
                            }
                        }
                    }
                }
                else {
                    cont = true;
                }
                if cont {
                    PostureModule::set_lr(fighter.module_accessor, opponent_lr);
                    PostureModule::update_rot_y_lr(fighter.module_accessor);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLAG_AUTO_TURN_END_STATUS);
                }
            }
        }
    }
    0.into()
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_DOLLY {
            return;
        }
        fighter.global_table[GUARD_CONT_UNIQ].assign(&L2CValue::Ptr(dolly_guard_cont_pre as *const () as _));
        fighter.global_table[CHECK_GROUND_ATTACK_UNIQ].assign(&L2CValue::Ptr(dolly_check_ground_attack_pre as *const () as _));
        fighter.global_table[CHECK_GROUND_SPECIAL_UNIQ].assign(&L2CValue::Ptr(dolly_check_ground_attack_pre as *const () as _));
        fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&L2CValue::Ptr(dolly_check_ground_attack_pre as *const () as _));
        fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(dolly_check_special_command as *const () as _));
        fighter.global_table[CHECK_GROUND_CATCH_UNIQ].assign(&L2CValue::Ptr(dolly_check_ground_catch_pre as *const () as _));
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(dolly_status_end_control as *const () as _));
        // fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Bool(false));
        fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(dolly_fgc as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
