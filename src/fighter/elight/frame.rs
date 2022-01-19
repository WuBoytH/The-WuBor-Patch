use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    super::super::element::helper::*,
    wubor_utils::{
        wua_bind::*,
        vars::*,
        table_const::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_ELIGHT )]
fn elight_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
            if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_FINAL
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_ELIGHT_STATUS_KIND_FINAL_READY
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_ELIGHT_STATUS_KIND_FINAL_SCENE01
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_ELIGHT_STATUS_KIND_FINAL_SCENE02
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_ELIGHT_STATUS_KIND_FINAL_END
            && MiscModule::is_damage_check(fighter.module_accessor, false) == false {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_hi_jump") {
            if PostureModule::lr(fighter.module_accessor) == 1.0 && ControlModule::get_stick_x(fighter.module_accessor) < -0.75 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
            else if PostureModule::lr(fighter.module_accessor) == -1.0 && ControlModule::get_stick_x(fighter.module_accessor) > 0.75 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
        }

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CANCEL)
        && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CANCEL);
            MotionModule::set_frame(fighter.module_accessor, 25.0, false);
        }

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD {
                if MotionModule::frame(fighter.module_accessor) >= 11.0
                && MotionModule::frame(fighter.module_accessor) < 32.0 {
                    if ControlModule::check_button_on(fighter.module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END, true);
                        WorkModule::on_flag(fighter.module_accessor, FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CANCEL);
                    }
                }
                else {
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CANCEL);
                }
            }
            element_fgc(fighter);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        elight_frame
    );
}