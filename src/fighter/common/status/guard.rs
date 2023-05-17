#![allow(non_snake_case)]

use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_Guard)]
unsafe fn status_pre_guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    let shield_eff = VarModule::get_int(fighter.battle_object, guard::int::SHIELD_EFF_ID);
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_FLOAT,
        *FS_SUCCEEDS_KEEP_VISIBILITY
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0
    );
    VarModule::set_int(fighter.battle_object, guard::int::SHIELD_EFF_ID, shield_eff);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_guard_cont_pre)]
unsafe fn sub_guard_cont_pre(fighter: &mut L2CFighterCommon) {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD);
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON
    && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_RUN {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
    // WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    // WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    // WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
    // WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    // WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    // if GroundModule::is_passable_ground(fighter.module_accessor) {
    //     WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
    // }
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
}

#[skyline::hook(replace = L2CFighterCommon_sub_guard_cont)]
unsafe fn sub_guard_cont(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[GUARD_CONT_UNIQ].get_bool() && {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[GUARD_CONT_UNIQ].get_ptr());
        callable(fighter).get_bool()
    } {
        return true.into();
    }
    let can_act = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME) <= 0;
    // let check_guard_hold = fighter.check_guard_hold().get_bool();
    let check_guard_hold = false;
    // if !check_guard_hold {
    //     if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
    //         return true.into();
    //     }
    // }
    if can_act
    && !check_guard_hold
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD)
    && ItemModule::is_have_item(fighter.module_accessor, 0) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        !fighter.pop_lua_stack(1).get_bool()
    } {
        if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
        || (fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER == 0
        && fighter.global_table[CMD_CAT3].get_i32() & (*FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI | *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI4) != 0) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                ControlModule::clear_command_one(fighter.module_accessor, 0, 0x1d); // FIGHTER_PAD_CMD_CAT1_CATCH
                VarModule::on_flag(fighter.battle_object, guard::flag::ADD_BUFFER);
                fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), false.into());
                return true.into();
            }
        }
    }
    if can_act
    && !check_guard_hold {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            ControlModule::clear_command_one(fighter.module_accessor, 0, 0x1d); // FIGHTER_PAD_CMD_CAT1_CATCH
            VarModule::on_flag(fighter.battle_object, guard::flag::ADD_BUFFER);
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), false.into());
            return true.into();
        }
        if fighter.sub_check_button_jump().get_bool()
        || fighter.sub_check_button_frick().get_bool() {
            ControlModule::clear_command_one(fighter.module_accessor, 0, 0x1d); // FIGHTER_PAD_CMD_CAT1_CATCH
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), false.into());
            return true.into();
        }
    }
    
    if can_act
    && fighter.check_guard_attack_special_hi(check_guard_hold.into()).get_bool() {
        return true.into();
    }
    if can_act
    && !check_guard_hold {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            ControlModule::clear_command_one(fighter.module_accessor, 0, 0x1d); // FIGHTER_PAD_CMD_CAT1_CATCH
            VarModule::on_flag(fighter.battle_object, guard::flag::ADD_BUFFER);
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), false.into());
            return true.into();
        }
        if fighter.sub_check_button_jump().get_bool()
        || fighter.sub_check_button_frick().get_bool() {
            VarModule::on_flag(fighter.battle_object, guard::flag::ADD_BUFFER);
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), false.into());
            return true.into();
        }
    }
    if !check_guard_hold {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_F != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_B != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
            return true.into();
        }
        // if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS)
        // && fighter.global_table[CMD_CAT2].get_i32() & (
        //     *FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI |
        //     *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R |
        //     *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW
        // ) != 0
        // && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        //     fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
        //     return true.into();
        // }
    }
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON
    && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_RUN {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) && {
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let lr = PostureModule::lr(fighter.module_accessor);
            let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
            stick_x * lr <= turn_run_stick_x
        } && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
            return true.into();
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && !ItemModule::is_have_item(fighter.module_accessor, 0) {
        if !can_act {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), true.into());
        }
        else {
            VarModule::on_flag(fighter.battle_object, guard::flag::ADD_BUFFER);
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), false.into());
        }
        return true.into();
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_guard_main_common)]
unsafe fn status_guard_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Shield Breaks no longer happen if you just hold Shield
    // let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    // if shield_hp < 0.0 {
    //     fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY.into(), false.into());
    //     return true.into();
    // }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        let min_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        if min_frame <= 0 {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_check_guard_attack_special_hi)]
unsafe fn check_guard_attack_special_hi(fighter: &mut L2CFighterCommon, guard_hold: L2CValue) -> L2CValue {
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if !guard_hold.get_bool() {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START)
        && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            ControlModule::clear_command_one(fighter.module_accessor, 0, 0x1d); // FIGHTER_PAD_CMD_CAT1_CATCH
            VarModule::on_flag(fighter.battle_object, guard::flag::ADD_BUFFER);
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), false.into());
            return true.into();
        }
    }
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let special_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_y"));
    let flick_y = fighter.global_table[FLICK_Y].get_i32();
    let jump_flick_y = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_flick_y"));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_SPECIAL_HI)
    || stick_y > special_stick_y && flick_y < jump_flick_y {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI)
        && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
            let cont = if fighter.global_table[CHECK_SPECIAL_HI_UNIQ].get_bool() {
                let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_SPECIAL_HI_UNIQ].get_ptr());
                callable(fighter).get_bool()
            }
            else {
                true
            };
            if cont {
                // fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
                VarModule::on_flag(fighter.battle_object, guard::flag::ADD_BUFFER);
                fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), false.into());
                return true.into();
            }
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_SPECIAL_HI);
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardFunc_updateShield)]
unsafe fn sub_ftstatusuniqprocessguardfunc_updateshield(fighter: &mut L2CFighterCommon, _param_1: L2CValue) {
    // There used to be code here for shield tilting, but nope not anymore
    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let scale = fighter.FighterStatusGuard__calc_shield_scale(shield_hp.into()).get_f32();
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: scale, y: scale, z: scale});
    let shield_eff = VarModule::get_int(fighter.battle_object, guard::int::SHIELD_EFF_ID) as u32;
    let shield_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    let shield_low_hp = shield_max * 0.4;
    if EffectModule::is_exist_effect(fighter.module_accessor, shield_eff) {
        let ratio_main = (shield_hp - shield_low_hp) / (shield_max - shield_low_hp);
        let ratio_sub = shield_hp / shield_low_hp;
        let alpha = 0.85 * ratio_main.clamp(0.0, 1.0) + 0.15 * ratio_sub.clamp(0.02, 1.0);
        EffectModule::set_alpha(fighter.module_accessor, shield_eff, alpha);
    }
    if shield_hp <= shield_low_hp
    && !VarModule::is_flag(fighter.battle_object, guard::flag::SET_SHIELD_LOW_SMOKE) {
        macros::EFFECT_FLW_POS(fighter, Hash40::new("sys_shield_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
        VarModule::on_flag(fighter.battle_object, guard::flag::SET_SHIELD_LOW_SMOKE);
    }
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_FighterStatusGuard__set_shield_scale)]
unsafe fn bind_address_call_fighterstatusguard__set_shield_scale(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent, param_1: L2CValue) -> L2CValue {
    fighter.FighterStatusGuard__set_shield_scale(param_1)
}

#[skyline::hook(replace = L2CFighterCommon_FighterStatusGuard__set_shield_scale)]
unsafe fn fighterstatusguard__set_shield_scale(fighter: &mut L2CFighterCommon, _param_1: L2CValue) -> L2CValue {
    // There used to be code here for shield tilting, but nope not anymore
    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let scale = fighter.FighterStatusGuard__calc_shield_scale(shield_hp.into()).get_f32();
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: scale, y: scale, z: scale});
    let shield_eff = VarModule::get_int(fighter.battle_object, guard::int::SHIELD_EFF_ID) as u32;
    let shield_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    let shield_low_hp = shield_max * 0.4;
    if EffectModule::is_exist_effect(fighter.module_accessor, shield_eff) {
        let ratio_main = (shield_hp - shield_low_hp) / (shield_max - shield_low_hp);
        let ratio_sub = shield_hp / shield_low_hp;
        let alpha = 0.85 * ratio_main.clamp(0.0, 1.0) + 0.15 * ratio_sub.clamp(0.02, 1.0);
        EffectModule::set_alpha(fighter.module_accessor, shield_eff, alpha);
    }
    if shield_hp <= shield_low_hp
    && !VarModule::is_flag(fighter.battle_object, guard::flag::SET_SHIELD_LOW_SMOKE) {
        macros::EFFECT_FLW_POS(fighter, Hash40::new("sys_shield_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
        VarModule::on_flag(fighter.battle_object, guard::flag::SET_SHIELD_LOW_SMOKE);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_guard,
            sub_guard_cont_pre,
            sub_guard_cont,
            status_guard_main_common,
            check_guard_attack_special_hi,
            sub_ftstatusuniqprocessguardfunc_updateshield,
            bind_address_call_fighterstatusguard__set_shield_scale,
            fighterstatusguard__set_shield_scale
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}