use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_Jump_sub)]
unsafe extern "C" fn status_jump_sub(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::fighter::instance::flag::SUPER_JUMP) {
        let mini_jump = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        if mini_jump {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_hyperhop"), true, false, false, false, enSEType(0));
        }
        else {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_superjump"), true, false, false, false, enSEType(0));
        }
    }
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&0xFE.into());
    ControlModule::reset_trigger(fighter.module_accessor);
    fighter.sub_air_check_fall_common_pre();
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_STOP_CEIL);
    let mot;
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_POWBLOCK_QUAKE_JUMP) {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let lr = PostureModule::lr(fighter.module_accessor);
        let jump_neutral_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_x"));
        let mini_jump = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        let stick_x = stick_x * lr * -1.0;
        if stick_x <= jump_neutral_x {
            if !mini_jump {
                mot = hash40("jump_f");
            }
            else {
                mot = hash40("jump_f_mini");
            }
        }
        else {
            if !mini_jump {
                mot = hash40("jump_b");
            }
            else {
                mot = hash40("jump_b_mini");
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
            SoundModule::play_se(
                fighter.module_accessor,
                Hash40::new("se_item_usagihat_jump_01"),
                true,
                false,
                false,
                false,
                enSEType(0)
            );
        }
    }
    else {
        mot = hash40("jump_f_mini");
    }
    let ret = if param_1.get_u64() != hash40("invalid") {
        param_1.get_u64()
    }
    else {
        mot
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(ret),
        0.0,
        1.0,
        false,
        param_2.get_f32(),
        false,
        false
    );
    if fighter.global_table[FALL_BRAKE_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[FALL_BRAKE_UNIQ].get_ptr());
        callable(fighter);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
    ret.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_Jump_Main)]
unsafe extern "C" fn status_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_stop_ceil().get_bool() {
        return 1.into();
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        fighter.sub_air_check_superleaf_fall_slowly();
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_Jump)]
unsafe extern "C" fn bind_address_call_status_end_jump(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_Jump()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Jump)]
unsafe extern "C" fn status_end_jump(_fighter: &mut L2CFighterCommon) -> L2CValue {
    // VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::SUPER_JUMP);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_jump_sub,
            status_jump_main,
            bind_address_call_status_end_jump,
            status_end_jump
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}