use super::*;

#[skyline::hook(replace = L2CFighterCommon_sub_cliff_catch_move_uniq_process_init_common)]
unsafe extern "C" fn sub_cliff_catch_move_uniq_process_init_common(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue) {
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_ENV_WIND,
        0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    if !GroundModule::is_status_cliff(fighter.module_accessor) {
        return;
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2dbe023abb));
    let hang_cliff_dir = GroundModule::hang_cliff_dir(fighter.module_accessor);
    PostureModule::set_lr(fighter.module_accessor, -hang_cliff_dir);
    GroundModule::set_shape_flag(fighter.module_accessor, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FRONT_FIX as u16, true);
    fighter.clear_lua_stack();
    lua_args!(fighter, param_2.get_hash());
    sv_fighter_util::adjust_joint_pos_change_motion(fighter.lua_state_agent, param_2.get_hash());
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    MotionModule::change_motion(
        fighter.module_accessor,
        param_1.get_hash(),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    // MotionModule::set_rate(fighter.module_accessor, 0.0);

    // Called a second time?
    GroundModule::set_shape_flag(fighter.module_accessor, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FRONT_FIX as u16, true);

    GroundModule::entry_cliff(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CLIFF_CATCH_MOVE);
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if prev_status == *FIGHTER_STATUS_KIND_AIR_LASSO_HANG
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_CATCH_MOVE_FLAG_AIR_LASSO_CATCH) {
        return;
    }
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUB_FIGHTER) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_CLIFF_NUM);
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_CliffCatchCommon)]
unsafe extern "C" fn sub_status_cliffcatchcommon(fighter: &mut L2CFighterCommon) {
    // let rate = FighterUtil::get_cliff_catch_motion_rate(fighter.module_accessor);
    // MotionModule::set_rate(fighter.module_accessor, rate);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RUN_FALL);
    FighterUtil::set_pickelblock_mode_ignoreandattack(fighter.module_accessor);
    GroundModule::set_ignore_boss(fighter.module_accessor, true);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_cliff_catch_move_uniq_process_init_common,
            sub_status_cliffcatchcommon
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}