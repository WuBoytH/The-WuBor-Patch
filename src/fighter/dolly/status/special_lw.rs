use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_speciallw_end_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_speciallw_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_speciallw_end_main(fighter)
}

unsafe extern "C" fn dolly_speciallw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK {
        VarModule::off_flag(fighter.battle_object, dolly::instance::flag::IS_SPECIAL_CANCEL);
    }
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_speciallw_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_attack"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let log = if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_07
    }
    else {
        let command_power_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("command_power_mul"));
        AttackModule::set_power_mul_status(fighter.module_accessor, command_power_mul);
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_08
    };
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), log - 1);
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_speciallw_attack_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_speciallw_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_LANDING_HEAVY) {
            let param = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
                hash40("landing_heavy_frame_command")
            }
            else {
                hash40("landing_heavy_frame")
            };
            let landing_heavy_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), param);
            WorkModule::set_int(fighter.module_accessor, landing_heavy_frame, *FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_LANDING_STIFFNESS_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return 0.into();
        }
        let frame = fighter.global_table[MOTION_FRAME].get_f32() as i32;
        let start_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_INT_START_FRAME);
        let attack_no_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("attack_no_landing_frame"));
        if frame + start_frame >= attack_no_landing_frame {
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_speciallw_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let param = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_HIT) {
        hash40("landing_frame_hit")
    }
    else {
        hash40("landing_frame_fail")
    };
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), param);
    WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    VarModule::off_flag(fighter.battle_object, dolly::instance::flag::IS_SPECIAL_CANCEL);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        dolly_speciallw_end,
        dolly_speciallw_command_end,
        dolly_speciallw_attack_main, dolly_speciallw_attack_end
    );
}