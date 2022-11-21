use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    wubor_utils::table_const::*
};

#[skyline::hook(replace = L2CFighterCommon_sub_status_RunBrake)]
unsafe fn sub_status_runbrake(fighter: &mut L2CFighterCommon) {
    let mut mot = hash40("run_brake");
    if MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("run_brake_l")) {
        mot = hash40("run_brake_r");
        let frame = MotionModule::frame(fighter.module_accessor);
        let run_left_start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("run_left_start_frame"));
        let run_left_end_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("run_left_end_frame"));
        if !fighter.check_run_brake_l_frame(run_left_start_frame.into(), run_left_end_frame.into(), frame.into()).get_bool() {
            let run_left_start_frame2 = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("run_left_start_frame2"));
            let run_left_end_frame2 = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("run_left_end_frame2"));
            if run_left_start_frame2 != 0.0
            && fighter.check_run_brake_l_frame(run_left_start_frame2.into(), run_left_end_frame2.into(), frame.into()).get_bool() {
                mot = hash40("run_brake_l");
            }
        }
        else {
            mot = hash40("run_brake_l");
        }
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(mot),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let groups = [
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP
    ];
    for term_group in groups.iter() {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *term_group);
    }
    let terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_RUN_BRAKE_HI4,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT // Squat during Run Brake
    ];
    for term in terms.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *term);
    }
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY);
    if fighter.global_table[FALL_BRAKE_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[FALL_BRAKE_UNIQ].get_ptr());
        callable(fighter);
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_length = sv_kinetic_energy::get_speed_length(fighter.lua_state_agent);
    WorkModule::set_float(fighter.module_accessor, speed_length, *FIGHTER_STATUS_RUN_BRAKE_WORK_FLOAT_START_SPEED);
    let shake_data_brake_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("shake_data_brake_scale"));
    ShakeModule::req(
        fighter.module_accessor,
        Hash40::new("brake"),
        10000,
        false,
        &Vector2f{x: 0.0, y: 0.0},
        shake_data_brake_scale,
        0.0,
        false,
        false
    );
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_RUN_BRAKE_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_run_brake_uniq_check(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_run_brake_uniq_check as *const () as _));
}

#[skyline::hook(replace = L2CFighterCommon_status_RunBrake_Main)]
unsafe fn status_runbrake_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }

    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return 0.into();
    }

    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_RUN_BRAKE_HI4)
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0
    && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } || {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() {
            ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0
        }
        else {
            false
        }
    } {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 0.into();
    }

    if fighter.sub_transition_group_check_ground_item().get_bool() {
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH)
    && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_DASH.into(), true.into());
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH)
    && ItemModule::is_have_item(fighter.module_accessor, 0)
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
    && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        !fighter.pop_lua_stack(1).get_bool()
    } {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH)
    && ItemModule::is_have_item(fighter.module_accessor, 0)
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
    && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        !fighter.pop_lua_stack(1).get_bool()
    } {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_DASH.into(), false.into());
        return 0.into();
    }

    if fighter.sub_transition_group_check_ground_catch().get_bool()
    || fighter.sub_transition_group_check_ground_escape().get_bool()
    || fighter.sub_transition_group_check_ground_special().get_bool() {
        return 0.into();
    }

    // Allow crouch out of run brake
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT)
    && fighter.sub_check_command_squat().get_bool() {
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT.into(), true.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_DASH.into(), true.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH)
    && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), true.into());
        return 1.into();
    }

    if fighter.sub_transition_group_check_ground_attack().get_bool() {
        return 0.into();
    }

    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    || FighterUtil::is_valid_auto_catch_item(fighter.module_accessor, false) {
        if cat1 & (
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
        ) != 0 && {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY);
            sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        }
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH)
        && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
            return true.into();
        }
        if ItemModule::get_pickable_item_size(fighter.module_accessor) == *ITEM_SIZE_LIGHT as u64
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM);
            sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
            return true.into();
        }
    }

    if fighter.sub_transition_group_check_ground_jump().get_bool() {
        return 0.into();
    }

    let stick_x = fighter.global_table[STICK_X].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);

    let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_RUN_BRAKE_FLAG_TURN_RUN)
    && stick_x * lr <= turn_run_stick_x {
        fighter.change_status(FIGHTER_STATUS_KIND_TURN_RUN.into(), false.into());
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) 
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
        return true.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH) 
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
        return true.into();
    }

    if fighter.sub_ground_check_ottotto_motion_end().get_bool() {
        return 0.into();
    }

    if !MotionModule::is_end(fighter.module_accessor) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_RUN_BRAKE_FLAG_STOP_SHAKE) {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_length = sv_kinetic_energy::get_speed_length(fighter.lua_state_agent);
            WorkModule::set_float(fighter.module_accessor, speed_length, *FIGHTER_STATUS_RUN_BRAKE_WORK_FLOAT_START_SPEED);
            let run_brake_stop_shake_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_brake_stop_shake_speed"));
            if speed_length < run_brake_stop_shake_speed {
                ShakeModule::stop_kind(fighter.module_accessor, Hash40::new("brake"));
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_RUN_BRAKE_FLAG_STOP_SHAKE);
            }
            else {
                let brake_start_speed = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_RUN_BRAKE_WORK_FLOAT_START_SPEED);
                let diff = brake_start_speed - run_brake_stop_shake_speed;
                let diff2 = speed_length - run_brake_stop_shake_speed;
                let shake_data_brake_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("shake_data_brake_scale"));
                let run_brake_stop_shake_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_brake_stop_shake_scale"));
                let ratio = diff2 / diff;
                let lerp = fighter.lerp(run_brake_stop_shake_scale.into(), 1.0_f32.into(), ratio.into()).get_f32();
                let mul = lerp * shake_data_brake_scale;
                ShakeModule::set_scale_kind(fighter.module_accessor, Hash40::new("brake"), mul);
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }

    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_status_runbrake,
            status_runbrake_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}