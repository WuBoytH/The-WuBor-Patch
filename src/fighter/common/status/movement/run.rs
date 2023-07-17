use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_RunBrake)]
unsafe fn status_pre_runbrake(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_RUN_STOP,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        true,
        false,
        false,
        0,
        (*FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT | *FIGHTER_STATUS_ATTR_INTO_DOOR) as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_Run_Sub)]
unsafe fn status_run_sub(fighter: &mut L2CFighterCommon) {
    let start_frame = if [
        *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH
    ].contains(&fighter.global_table[PREV_STATUS_KIND].get_i32()) {
        WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_RUN_WORK_FLOAT_START_FRAME)
    }
    else {
        0.0
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("run"),
        start_frame,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let groups = [
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP
    ];
    for term_group in groups.iter() {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *term_group);
    }
    let terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN
    ];
    for term in terms.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *term);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_GEKIKARA_RUN_BRAKE);
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_Run_Main)]
unsafe fn status_run_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[RUN_MAIN_UNIQ].get_bool() && {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[RUN_MAIN_UNIQ].get_ptr());
        callable(fighter).get_bool()
    } {
        return 1.into();
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER) {
        fighter.change_status(FIGHTER_STATUS_KIND_HAMMER_WALK.into(), false.into());
        return 1.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return 0.into();
    }

    let stick_x = fighter.global_table[STICK_X].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);

    let some_threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x206138766c);

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && stick_x * lr < some_threshold {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        let mut throw = fighter.pop_lua_stack(1).get_bool();
        if !throw {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
            sv_module_access::item(fighter.lua_state_agent);
            if fighter.pop_lua_stack(1).get_bool() {
                throw = ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0;
            }
        }
        if throw {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            return 0.into();
        }
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_DASH.into(), false.into());
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH)
    && ItemModule::is_have_item(fighter.module_accessor, 0)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
    && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        !fighter.pop_lua_stack(1).get_bool()
    } {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_DASH.into(), false.into());
        return 0.into();
    }

    let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN)
    && WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_RUN_WORK_FLOAT_RUN_FRAME) <= 0.0
    && stick_x * lr <= turn_run_stick_x
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
    && !ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
    && !ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
        return 0.into();
    }

    if fighter.sub_transition_group_check_ground_guard().get_bool() {
        return 0.into();
    }

    if fighter.sub_transition_group_check_ground_special().get_bool() {
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH)
    && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_DASH.into(), false.into());
        return 0.into();
    }

    if fighter.sub_transition_specialflag_hoist().get_bool() {
        return 1.into();
    }

    // Allow crouch out of dash
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT)
    && fighter.sub_check_command_squat().get_bool() {
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT.into(), true.into());
        return 0.into();
    }

    if fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_bool() && {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_ptr());
        callable(fighter).get_bool()
    } {
        return 0.into();
    }

    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), true.into());
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), true.into());
        return 0.into();
    }

    if fighter.sub_transition_group_check_ground_jump().get_bool() {
        return 0.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN)
    && WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_RUN_WORK_FLOAT_RUN_FRAME) <= 0.0
    && stick_x * lr <= turn_run_stick_x {
        fighter.change_status(FIGHTER_STATUS_KIND_TURN_RUN.into(), true.into());
        return 0.into();
    }

    let run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_stick_x"));
    let walk_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_stick_x"));

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA) {
        if stick_x.abs() < run_stick_x {
            fighter.change_status(FIGHTER_STATUS_KIND_RUN_BRAKE.into(), true.into());
            return 0.into();
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_GEKIKARA_RUN_BRAKE)
        && stick_x.abs() < walk_stick_x {
            fighter.change_status(FIGHTER_STATUS_KIND_RUN_BRAKE.into(), true.into());
            return 0.into();
        }
    }

    fighter.sub_ground_check_stop_wall();

    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_runbrake,
            status_run_sub,
            status_run_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}