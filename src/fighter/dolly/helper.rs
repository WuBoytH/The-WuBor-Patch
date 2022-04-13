use {
    smash::{
        lua2cpp::{L2CFighterCommon, L2CAgentBase},
        hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::table_const::*,
    super::{vl, vars::*}
};

pub unsafe extern "C" fn dolly_hit_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        let situation : L2CValue;
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            situation = SITUATION_KIND_AIR.into();
        }
        else {
            situation = SITUATION_KIND_GROUND.into();
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT)
        && dolly_final_cancel(fighter, situation.clone()).get_bool() {
            return 1.into();
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT)
        && dolly_special_cancel(fighter, situation.clone()).get_bool() {
            return 1.into();
        }
    }
    0.into()
}

pub unsafe extern "C" fn dolly_special_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue {
    let ret;
    let terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW
    ];
    let mut enableds = [false; 13];
    for x in 0..terms.len() {
        enableds[x] = WorkModule::is_enable_transition_term(fighter.module_accessor, terms[x]);
    }
    for val in terms.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *val);
    }
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
    }
    if situation.get_i32() != *SITUATION_KIND_GROUND {
        ret = fighter.sub_transition_group_check_air_special();
    }
    else {
        ret = fighter.sub_transition_group_check_ground_special();
    }
    for x in 0..terms.len() {
        if !enableds[x] {
            WorkModule::unable_transition_term(fighter.module_accessor, terms[x]);
        }
    }
    if ret.get_bool() {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
    }
    ret
}

pub unsafe extern "C" fn dolly_final_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue {
    let ret;
    let final_cancel = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    if situation.get_i32() != *SITUATION_KIND_GROUND {
        ret = fighter.sub_transition_group_check_air_special();
    }
    else {
        ret = fighter.sub_transition_group_check_ground_special();
    }
    if !final_cancel {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    }
    ret
}

pub unsafe extern "C" fn dolly_attack_start_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.global_table[MOTION_FRAME].get_f32() <= WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("attack_start_cancel_frame")) as f32
    && dolly_kara_cancel(fighter).get_bool() {
        return 1.into();
    }
    0.into()
}

pub unsafe extern "C" fn dolly_kara_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    let terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2
    ];
    let mut enableds = [false; 6];
    for x in 0..terms.len() {
        enableds[x] = WorkModule::is_enable_transition_term(fighter.module_accessor, terms[x]);
    }
    for val in terms.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *val);
    }
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
    }
    let val = fighter.sub_transition_group_check_special_command();
    for x in 0..terms.len() {
        if !enableds[x] {
            WorkModule::unable_transition_term(fighter.module_accessor, terms[x]);
        }
    }
    val
}

pub struct SpecialCancelStats {
    pub dmg: f32,
    pub bkb: i32
}

pub unsafe fn dolly_calc_special_cancel(fighter: &mut L2CAgentBase, mut dmg: f32, mut bkb: i32) -> SpecialCancelStats {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL) {
        dmg *= vl::param_private::special_cancel_damage_mul;
        bkb = (bkb as f32 * vl::param_private::special_cancel_bkb_mul) as i32;
    }
    SpecialCancelStats{dmg: dmg, bkb: bkb}
}
