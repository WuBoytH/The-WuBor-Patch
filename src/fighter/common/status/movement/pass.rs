use crate::imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_Pass_common)]
unsafe extern "C" fn status_pass_common(fighter: &mut L2CFighterCommon) {
    fighter.sub_air_check_fall_common_pre();
    let transitions = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON
    ];
    for val in transitions.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *val);
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("pass"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
}

#[skyline::hook(replace = L2CFighterCommon_status_Pass_Main_sub)]
unsafe extern "C" fn status_pass_main_sub(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    let pass_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_PASS_WORK_INT_FRAME);
    if pass_frame == 0 {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_PASS_FLAG_IS_SET_PASS) {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(param_1.get_ptr());
            if callable(fighter).get_bool() {
                return 0.into();
            }
        }
        if !fighter.sub_air_check_fall_common().get_bool() {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
        return 0.into();
    }
    if pass_frame <= 0 {
        return 0.into();
    }
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool()
    || fighter.sub_transition_group_check_ground_jump().get_bool() {
        return 1.into();
    }
    if fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].get_ptr());
        callable(fighter).get_bool();
    }
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
            if fighter.global_table[CHECK_SPECIAL_LW_UNIQ].get_bool() && {
                let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_SPECIAL_LW_UNIQ].get_ptr());
                callable(fighter).get_bool()
            } {
                return 1.into();
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
                return 1.into();
            }
        }
    }
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE)
    && ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        }
    }
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) {
        let mut throw;
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        throw = fighter.pop_lua_stack(1).get_bool();
        if !throw {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
            sv_module_access::item(fighter.lua_state_agent);
            if !fighter.pop_lua_stack(1).get_bool() {
                throw = false;
            }
            else {
                throw = ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0;
            }
        }
        if throw {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            return 1.into();
        }
    }
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let attack_lw4_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw4_stick_y"));
    if stick_y <= attack_lw4_stick_y {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if fighter.global_table[CHECK_ATTACK_LW4_UNIQ].get_bool() && {
                let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_LW4_UNIQ].get_ptr());
                callable(fighter).get_bool()
            } {
                return 1.into();
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4_START.into(), true.into());
                return 1.into();
            }
        }
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pass_common,
            status_pass_main_sub
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}