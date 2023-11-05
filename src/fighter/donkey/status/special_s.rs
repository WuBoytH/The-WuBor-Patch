use {
    crate::imports::status_imports::*,
    super::super::vl
};

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn donkey_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if !barrel_check(fighter.module_accessor) {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r"), 1.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_l"), 1.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s").into(), Hash40::new("special_air_s").into(), false.into());
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(donkey_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) != hash40("appeal_lw_r")
    && MotionModule::motion_kind(fighter.module_accessor) != hash40("appeal_lw_l")
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
        }
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool()
        && !fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    0.into()
}

/// Checks how many barrels are on the screen.
/// If there are more than (# of owned barrels allowed)
/// existing at a time, DK will be unable to pull out a barrel.
pub unsafe fn barrel_check(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    let itemmanager = smash_rs::app::ItemManager::instance().unwrap();
    smash_rs::app::ItemManager::get_num_of_ownered_item(itemmanager, (*module_accessor).battle_object_id, smash_rs::app::ItemKind::Barrel)
    <
    vl::param_special_s::barrel_count as usize
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_S, donkey_special_s_main)
}