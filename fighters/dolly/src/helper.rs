use super::*;

#[skyline::from_offset(0x970ff0)]
pub unsafe extern "C" fn dolly_check_super_special_pre(module_accessor: *mut BattleObjectModuleAccessor, param_2: u8);

#[no_mangle]
pub extern "C" fn add_go(module_accessor: *mut BattleObjectModuleAccessor, amount: f32) {
    unsafe{
        if !VarModule::is_flag(module_accessor, vars::dolly::status::flag::DISABLE_METER_GAIN) {
            let meter_max = 200.0;
            let meter_const = vars::dolly::instance::float::GO_METER;
            let mut add_amount = amount;
            add_amount /= 1.0;
            FGCModule::update_meter(module_accessor, add_amount, meter_max, meter_const);
            let is_superspecial = !WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
            dolly_check_super_special_pre(module_accessor, is_superspecial as u8);
        }
    }
}

pub unsafe extern "C" fn dolly_hit_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        let situation : L2CValue = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            SITUATION_KIND_AIR.into()
        }
        else {
            SITUATION_KIND_GROUND.into()
        };
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT)
        && dolly_final_cancel(fighter, situation.clone()).get_bool() {
            return 1.into();
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT)
        && dolly_special_cancel(fighter, situation).get_bool() {
            return 1.into();
        }
    }
    0.into()
}

pub unsafe extern "C" fn dolly_special_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue {
    let mut terms = vec![
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
        // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
        // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2
    ];
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_DASH
    || fighter.global_table[STATUS_KIND].get_i32() == vars::dolly::status::ATTACK_DASH_COMMAND {
        if VarModule::is_flag(fighter.module_accessor, vars::dolly::instance::flag::RISING_FORCE) {
            terms = vec![*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND];
        }
        else {
            terms.remove(5);
        }
    }
    let mut enableds = [false; 13];
    for x in 0..terms.len() {
        enableds[x] = WorkModule::is_enable_transition_term(fighter.module_accessor, terms[x]);
    }
    for val in terms.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *val);
    }
    let ret = if situation.get_i32() != *SITUATION_KIND_GROUND {
        fighter.sub_transition_group_check_air_special()
    }
    else {
        fighter.sub_transition_group_check_ground_special()
    };
    for x in 0..terms.len() {
        if !enableds[x] {
            WorkModule::unable_transition_term(fighter.module_accessor, terms[x]);
        }
    }
    if ret.get_bool() {
        VarModule::on_flag(fighter.module_accessor, vars::dolly::instance::flag::SPECIAL_CANCEL);
    }
    ret
}

pub unsafe extern "C" fn dolly_final_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue {
    let final_cancel = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    let super_special = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
    let super_special2 = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_DASH
    || fighter.global_table[STATUS_KIND].get_i32() == vars::dolly::status::ATTACK_DASH_COMMAND {
        if VarModule::is_flag(fighter.module_accessor, vars::dolly::instance::flag::RISING_FORCE) {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
        }
    }
    let ret = if situation.get_i32() != *SITUATION_KIND_GROUND {
        fighter.sub_transition_group_check_air_special()
    }
    else {
        fighter.sub_transition_group_check_ground_special()
    };
    if !final_cancel {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    }
    if !super_special {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
    }
    if !super_special2 {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
    }
    ret
}

pub unsafe extern "C" fn dolly_attack_start_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.global_table[STATUS_FRAME].get_f32() <= WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("attack_start_cancel_frame")) as f32
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
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
        // WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
        // WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
        // WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW);
    }
    let val = fighter.sub_transition_group_check_special_command();
    for x in 0..terms.len() {
        if !enableds[x] {
            WorkModule::unable_transition_term(fighter.module_accessor, terms[x]);
        }
    }
    val
}

#[repr(C)]
pub struct SpecialCancelStats {
    pub dmg: f32,
    pub bkb: i32
}

pub unsafe extern "C" fn dolly_calc_special_cancel(fighter: &mut L2CAgentBase, mut dmg: f32, mut bkb: i32) -> SpecialCancelStats {
    if VarModule::is_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL) {
        let special_cancel_damage_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_misc"), hash40("special_cancel_damage_mul"));
        let special_cancel_bkb_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_misc"), hash40("special_cancel_bkb_mul"));
        dmg *= special_cancel_damage_mul;
        bkb = (bkb as f32 * special_cancel_bkb_mul) as i32;
    }
    SpecialCancelStats{dmg, bkb}
}

pub unsafe extern "C" fn dolly_status_end_control_lr_status_check(status: L2CValue) -> L2CValue {
    if [
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_RYU_STATUS_KIND_DASH_BACK,
        *FIGHTER_STATUS_KIND_RUN_BRAKE,
        *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE,
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV,
        *FIGHTER_STATUS_KIND_GUARD_OFF,
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
        *FIGHTER_STATUS_KIND_ESCAPE,
        *FIGHTER_STATUS_KIND_ESCAPE_F,
        *FIGHTER_STATUS_KIND_ESCAPE_B,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_CATCH,
        *FIGHTER_STATUS_KIND_CATCH_DASH,
        *FIGHTER_STATUS_KIND_CATCH_TURN,
        *FIGHTER_STATUS_KIND_CATCH_CUT,
        *FIGHTER_STATUS_KIND_THROW,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DOWN_STAND,
        *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK,
        *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
        *FIGHTER_STATUS_KIND_PASSIVE,
        *FIGHTER_STATUS_KIND_PASSIVE_FB,
        *FIGHTER_STATUS_KIND_FURAFURA_END,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG_END,
        *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
        *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
        *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
        *FIGHTER_STATUS_KIND_SLIP_STAND,
        *FIGHTER_STATUS_KIND_SLIP_STAND_ATTACK,
        *FIGHTER_STATUS_KIND_SLIP_STAND_F,
        *FIGHTER_STATUS_KIND_SLIP_STAND_B,
        *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP,
        *FIGHTER_STATUS_KIND_ITEM_THROW,
        *FIGHTER_STATUS_KIND_ITEM_THROW_DASH,
        *FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY,
        *FIGHTER_STATUS_KIND_ITEM_SWING,
        *FIGHTER_STATUS_KIND_ITEM_SWING_S3,
        *FIGHTER_STATUS_KIND_ITEM_SWING_S4,
        *FIGHTER_STATUS_KIND_ITEM_SWING_DASH,
        *FIGHTER_STATUS_KIND_APPEAL,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_LANDING,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_LANDING,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW,
        *FIGHTER_STATUS_KIND_FINAL,
        *FIGHTER_DOLLY_STATUS_KIND_FINAL_END
    ].contains(&status.get_i32()) {
        true.into()
    }
    else {
        false.into()
    }
}
