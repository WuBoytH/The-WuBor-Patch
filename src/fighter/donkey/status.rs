use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::{
        vars::*,
        table_const::*
    }
};

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn donkey_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if !barrel_check() {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r"), 1.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_l"), 1.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40(Hash40::new("special_s")), L2CValue::Hash40(Hash40::new("special_air_s")), L2CValue::Bool(false));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(donkey_specials_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) != hash40("appeal_lw_r")
    && MotionModule::motion_kind(fighter.module_accessor) != hash40("appeal_lw_l") {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
            }
            else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
            }
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
        if fighter.sub_wait_ground_check_common(L2CValue::I32(112)).get_bool() == false
        && fighter.sub_air_check_fall_common().get_bool() == false {
            return L2CValue::I32(1);
        }
    }
    L2CValue::I32(0)
}

pub unsafe fn barrel_check() -> bool {
    if smash::app::lua_bind::ItemManager::get_num_of_active_item(*ITEM_KIND_BARREL) >= 1 * DK_COUNT {
        return false;
    }
    return true;
}

pub fn install() {
    install_status_scripts!(
        donkey_specials_main
    );
}