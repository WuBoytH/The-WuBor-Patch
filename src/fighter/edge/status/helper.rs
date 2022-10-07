use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

pub unsafe extern "C" fn edge_special_hi_param_int_helper(fighter: &mut L2CFighterCommon, hash: L2CValue, charged_rush: L2CValue) -> L2CValue {
    let param = edge_special_hi_param_helper_inner(hash, charged_rush).get_u64();
    WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), param).into()
}

pub unsafe extern "C" fn edge_special_hi_param_float_helper(fighter: &mut L2CFighterCommon, hash: L2CValue, charged_rush: L2CValue) -> L2CValue {
    let param = edge_special_hi_param_helper_inner(hash, charged_rush).get_u64();
    WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), param).into()
}

pub unsafe extern "C" fn edge_special_hi_param_helper_inner(hash: L2CValue, charged_rush: L2CValue) -> L2CValue {
    let hash = hash.get_u64();
    if !charged_rush.get_bool() {
        return hash.into();
    }
    let new_hash = if hash == hash40("rot_decide_frame") {
        hash40("charged_rot_decide_frame")
    }
    else if hash == hash40("rot_end_frame") {
        hash40("charged_rot_end_frame")
    }
    else if hash == hash40("rush_frame") {
        hash40("charged_rush_frame")
    }
    else if hash == hash40("rush_speed") {
        hash40("charged_rush_speed")
    }
    else if hash == hash40("rush_brake_frame") {
        hash40("charged_rush_brake_frame")
    }
    else if hash == hash40("rush_brake") {
        hash40("charged_rush_brake")
    }
    else if hash == hash40("ground_speed_x_mul") {
        hash40("charged_ground_speed_x_mul")
    }
    else if hash == hash40("landing_speed_x_mul") {
        hash40("charged_landing_speed_x_mul")
    }
    else if hash == hash40("landing_brake_x") {
        hash40("charged_landing_brake_x")
    }
    else if hash == hash40("landing_fix_frame") {
        hash40("charged_landing_fix_frame")
    }
    else if hash == hash40("rotate_back_begin_frame") {
        hash40("charged_rotate_back_begin_frame")
    }
    else if hash == hash40("rotate_back_end_frame") {
        hash40("charged_rotate_back_end_frame")
    }
    else if hash == hash40("rush_end_speed_mul") {
        hash40("charged_rush_end_speed_mul")
    }
    else if hash == hash40("rush_end_brake_x") {
        hash40("charged_rush_end_brake_x")
    }
    else if hash == hash40("rush_end_gravity_accel") {
        hash40("charged_rush_end_gravity_accel")
    }
    else if hash == hash40("control_accel_x_mul") {
        hash40("charged_control_accel_x_mul")
    }
    else if hash == hash40("control_speed_x_max_mul") {
        hash40("charged_control_speed_x_max_mul")
    }
    else{
        hash
    };
    new_hash.into()
}

pub unsafe extern "C" fn edge_special_hi_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, edge::status::flag::SPECIAL_HI_CANCEL)
    && VarModule::get_int(fighter.battle_object, edge::instance::int::SPECIAL_HI_CANCEL_COUNT) < 2 {
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
            VarModule::inc_int(fighter.battle_object, edge::instance::int::SPECIAL_HI_CANCEL_COUNT);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
            return true.into();
        }
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
        if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool()
        || fighter.sub_transition_group_check_ground_jump().get_bool()
        || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 && fighter.sub_transition_group_check_air_jump_attack().get_bool())
        || fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
            VarModule::set_int(fighter.battle_object, edge::instance::int::SPECIAL_HI_CANCEL_COUNT, 2);
            return true.into();
        }
    }
    false.into()
}
