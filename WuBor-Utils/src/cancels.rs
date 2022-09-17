use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    }
};

pub unsafe fn jump_cancel_common(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue {
    let mut ret;
    let terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR
    ];
    let mut enableds = [false; 5];
    for x in 0..terms.len() {
        enableds[x] = WorkModule::is_enable_transition_term(fighter.module_accessor, terms[x]);
        WorkModule::enable_transition_term(fighter.module_accessor, terms[x]);
    }
    if situation.get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[0x20].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
            ret = fighter.sub_transition_group_check_air_jump_attack().get_bool();
        }
        else {
            ret = fighter.sub_transition_group_check_air_jump_aerial().get_bool();
        }
    }
    else {
        ret = fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool();
        if !ret {
            ret = fighter.sub_transition_group_check_ground_jump().get_bool();
        }
    }
    for x in 0..terms.len() {
        if !enableds[x] {
            WorkModule::unable_transition_term(fighter.module_accessor, terms[x]);
        }
    }
    ret.into()
}

pub unsafe fn airdash_cancel_common(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue {
    let ret;
    let terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR
    ];
    let mut enableds = [false; 1];
    for x in 0..terms.len() {
        enableds[x] = WorkModule::is_enable_transition_term(fighter.module_accessor, terms[x]);
        WorkModule::enable_transition_term(fighter.module_accessor, terms[x]);
    }
    if situation.get_i32() != *SITUATION_KIND_GROUND {
        ret = fighter.sub_transition_group_check_air_escape().get_bool();
    }
    else {
        ret = false;
    }
    for x in 0..terms.len() {
        if !enableds[x] {
            WorkModule::unable_transition_term(fighter.module_accessor, terms[x]);
        }
    }
    ret.into()
}

pub unsafe fn special_cancel_common(fighter: &mut L2CFighterCommon, situation: L2CValue, allowed_terms: Vec<i32>) -> L2CValue {
    let ret;
    let terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
    ];
    let mut enableds = [false; 12];
    for x in 0..terms.len() {
        enableds[x] = WorkModule::is_enable_transition_term(fighter.module_accessor, terms[x]);
    }
    for val in allowed_terms.iter() {
        if terms.contains(val) {
            WorkModule::enable_transition_term(fighter.module_accessor, *val);
        }
    }
    if situation.get_i32() != *SITUATION_KIND_GROUND {
        ret = fighter.sub_transition_group_check_air_special().get_bool();
    }
    else {
        ret = fighter.sub_transition_group_check_ground_special().get_bool();
    }
    for x in 0..terms.len() {
        if !enableds[x] {
            WorkModule::unable_transition_term(fighter.module_accessor, terms[x]);
        }
    }
    ret.into()
}

pub unsafe fn aerial_cancel_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret;
    let attack_air = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    ret = fighter.sub_transition_group_check_air_attack().get_bool();
    if !attack_air {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    }
    ret.into()
}

pub unsafe fn aerial_cancel_common_revised(fighter: &mut L2CFighterCommon, allow_jumps: bool) -> L2CValue {
    let ret;
    let attack_air = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    let terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON
    ];
    let mut enableds = [false; 3];
    if allow_jumps {
        for x in 0..terms.len() {
            enableds[x] = WorkModule::is_enable_transition_term(fighter.module_accessor, terms[x]);
            WorkModule::enable_transition_term(fighter.module_accessor, terms[x]);
        }
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    ret = fighter.sub_transition_group_check_air_attack().get_bool();
    if !attack_air {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    }
    for x in 0..terms.len() {
        if !enableds[x] {
            WorkModule::unable_transition_term(fighter.module_accessor, terms[x]);
        }
    }
    ret.into()
}

pub unsafe fn normal_cancel_common(fighter: &mut L2CFighterCommon, allowed_terms: Vec<i32>) -> L2CValue {
    let ret;
    let terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_3,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S3,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4
    ];
    let mut enableds = [false; 13];
    for x in 0..terms.len() {
        enableds[x] = WorkModule::is_enable_transition_term(fighter.module_accessor, terms[x]);
    }
    for val in allowed_terms.iter() {
        if terms.contains(val) {
            WorkModule::enable_transition_term(fighter.module_accessor, *val);
        }
    }
    ret = fighter.sub_transition_group_check_ground_attack().get_bool();
    for x in 0..terms.len() {
        if !enableds[x] {
            WorkModule::unable_transition_term(fighter.module_accessor, terms[x]);
        }
    }
    ret.into()
}
