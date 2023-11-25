use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_get_cliff_wait_hit_xlu_frame)]
unsafe extern "C" fn get_cliff_wait_hit_xlu_frame(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_xlu_max_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_air_xlu_max_frame")
    ) as f32;
    let air_xlu_max_air_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_air_xlu_max_air_frame")
    ) as f32;
    let damage_xlu_max_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_damage_xlu_max_frame")
    ) as f32;
    let damage_xlu_max_damage = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_damage_xlu_max_damage")
    ) as f32;
    let xlu_min_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_xlu_min_frame")
    ) as f32;

    // Vanilla Ultimate's Ledge Intangibility Formula
    let air_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) as f32;
    let xlu_from_air = (air_xlu_max_frame * (air_frame / air_xlu_max_air_frame)).clamp(0.0, air_xlu_max_frame);
    // println!("xlu from air: {}", xlu_from_air);
    let damage = DamageModule::damage(fighter.module_accessor, 0);
    let xlu_from_damage = (damage_xlu_max_frame - ((damage / damage_xlu_max_damage) * damage_xlu_max_frame)).clamp(0.0, damage_xlu_max_frame);
    // println!("xlu from damage: {}", xlu_from_damage);
    let xlu_frame = xlu_from_air + xlu_from_damage;
    let cliff_hang_invincible_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("cliff_hang_invincible_frame"), 0);
    // println!("cliff hang invincible frame? {}", cliff_hang_invincible_frame);
    // println!("final xlu: {}", (xlu_frame * cliff_hang_invincible_frame).clamp(xlu_min_frame, f32::MAX));
    ((xlu_frame * cliff_hang_invincible_frame).clamp(xlu_min_frame, f32::MAX)).into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_cliff_uniq_process_exit_Common)]
unsafe extern "C" fn sub_cliff_uniq_process_exit_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF) {
        let cliff_no_catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_no_catch_frame"));
        WorkModule::set_int(fighter.module_accessor, cliff_no_catch_frame, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME);
        if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_FALL {
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
        }
        else {
            VarModule::on_flag(fighter.module_accessor, fighter::instance::flag::LEDGE_INTANGIBILITY);
        }
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF);
    if param_1.get_bool() {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF);
        GroundModule::leave_cliff(fighter.module_accessor);
        if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_FALL {
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
        }
        else {
            VarModule::on_flag(fighter.module_accessor, fighter::instance::flag::LEDGE_INTANGIBILITY);
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_CliffJump1)]
unsafe extern "C" fn status_cliffjump1(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_jump_command_life = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
    if stick_jump_command_life == 0
    || fighter.global_table[FLICK_Y_DIR].get_i32() <= 0 {
        VarModule::on_flag(fighter.module_accessor, cliff::flag::CLIFF_JUMP_BUTTON);
        if ControlModule::is_jump_mini_button(fighter.module_accessor) {
            VarModule::on_flag(fighter.module_accessor, cliff::flag::CLIFF_JUMP_MINI);
        }
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);

    ControlModule::clear_command(fighter.module_accessor, false);
    ControlModule::reset_trigger(fighter.module_accessor);
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&L2CValue::I32(0xfe));
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_JUMP);
    fighter.set_cliff_xlu_frame(hash40("cliff_jump_quick1").into());
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("cliff_jump_quick1"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUB_FIGHTER) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_CLIFF_JUMP_NUM);
    }
    FighterUtil::set_pickelblock_mode_ignoreandattack(fighter.module_accessor);
    GroundModule::set_ignore_boss(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CliffJump1_Main as *const () as _))
}

pub unsafe extern "C" fn sub_cliff_jump1_uniq_process_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.module_accessor, cliff::flag::CLIFF_JUMP_BUTTON) {
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let jump_neutral_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_y"));
        if stick_y < jump_neutral_y {
            if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)
            && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                VarModule::on_flag(fighter.module_accessor, cliff::flag::CLIFF_JUMP_MINI);
                return 0.into();
            }
        }
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::on_flag(fighter.module_accessor, cliff::flag::CLIFF_JUMP_MINI);
        }
    }
    else {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
        || ControlModule::is_jump_mini_button(fighter.module_accessor) {
            VarModule::on_flag(fighter.module_accessor, cliff::flag::CLIFF_JUMP_MINI);
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_CliffJump2)]
unsafe extern "C" fn status_cliffjump2(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, cliff::flag::CLIFF_JUMP_MINI) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y * 0.6
        );
    }

    fighter.sub_air_check_fall_common_pre();

    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_STOP_CEIL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LASSO);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("cliff_jump_quick2"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_cliff_jump2_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_cliff_jump2_uniq as *const () as _));

    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CliffJump2_Main as *const () as _))
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            get_cliff_wait_hit_xlu_frame,
            sub_cliff_uniq_process_exit_common,
            status_cliffjump1,
            status_cliffjump2
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}