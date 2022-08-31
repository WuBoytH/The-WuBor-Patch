use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{cancels::*, vars::*, table_const::*},
    super::super::helper::*
};

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucario_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_start") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_start") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    WorkModule::set_customize_no(fighter.module_accessor, 0, 0);
    // lucario_drain_aura(fighter, false);
    lucario_special_set_kinetic(fighter);
    lucario_special_n_joint_translate(fighter);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_end = MotionModule::is_end(fighter.module_accessor);
    if !is_end {
        if !StatusModule::is_changing(fighter.module_accessor)
        && StatusModule::is_situation_changed(fighter.module_accessor) {
            lucario_special_set_kinetic(fighter);
            return 0.into();
        }
    }
    else {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && VarModule::get_int(fighter.battle_object, lucario::instance::int::AURA_LEVEL) > 2 {
            lucario_drain_aura(fighter, true);
            VarModule::on_flag(fighter.battle_object, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB);
            if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                WorkModule::set_customize_no(fighter.module_accessor, 1, 0);
            }
            else {
                WorkModule::set_customize_no(fighter.module_accessor, 2, 0);
            }
        }
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn lucario_special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    lucario_special_n_save_charge_status(fighter);
    0.into()
}

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucario_special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_hold") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_hold") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    ArticleModule::change_status(
        fighter.module_accessor,
        *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL,
        *WEAPON_LUCARIO_AURABALL_STATUS_KIND_CHARGE,
        ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)
    );
    lucario_special_set_kinetic(fighter);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_n_hold_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        lucario_special_set_kinetic(fighter);
        return 0.into();
    }
    let max_charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
    if charge >= max_charge_frame as i32 {
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn lucario_special_n_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    lucario_special_n_save_charge_status(fighter);
    0.into()
}

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucario_special_n_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_shoot") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_shoot") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
    let max_charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
    if max_charge_frame < charge as f32 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_FLAG_CHARGE_MAX);
    }
    lucario_special_set_kinetic(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_n_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_n_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if VarModule::get_int(fighter.battle_object, lucario::instance::int::AURA_LEVEL) > 0
    && VarModule::is_flag(fighter.battle_object, lucario::status::flag::SPECIAL_N_ENABLE_SUPERDASH) {
        let situation = fighter.global_table[SITUATION_KIND].clone();
        special_cancel_common(fighter, situation, [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI].to_vec());
    }
    if !MotionModule::is_end(fighter.module_accessor){
        if !StatusModule::is_changing(fighter.module_accessor)
        && StatusModule::is_situation_changed(fighter.module_accessor) {
            lucario_special_set_kinetic(fighter);
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        lucario_special_n_main, lucario_special_n_end,
        lucario_special_n_hold_main, lucario_special_n_hold_end,
        lucario_special_n_shoot_main
    );
}