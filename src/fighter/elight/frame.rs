use {
    smash::{
        lua2cpp::*,
        hash40,
        app::lua_bind::*,
        lib::{lua_const::*, *}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*
};

#[fighter_frame( agent = FIGHTER_KIND_ELIGHT, main )]
fn elight_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_hi_jump") {
            if (PostureModule::lr(fighter.module_accessor) == 1.0 && ControlModule::get_stick_x(fighter.module_accessor) < -0.75)
            || (PostureModule::lr(fighter.module_accessor) == -1.0 && ControlModule::get_stick_x(fighter.module_accessor) > 0.75) {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
        }

        if VarModule::is_flag(fighter.battle_object, elight::instance::flag::SPECIAL_S_CANCEL)
        && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END {
            VarModule::off_flag(fighter.battle_object, elight::instance::flag::SPECIAL_S_CANCEL);
            MotionModule::set_frame(fighter.module_accessor, 25.0, false);
        }

        if VarModule::is_flag(fighter.battle_object, fighter::instance::flag::IS_FGC) {
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD {
                if MotionModule::frame(fighter.module_accessor) >= 11.0
                && MotionModule::frame(fighter.module_accessor) < 32.0 {
                    if ControlModule::check_button_on(fighter.module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END, true);
                        VarModule::on_flag(fighter.battle_object, elight::instance::flag::SPECIAL_S_CANCEL);
                    }
                }
                else {
                    VarModule::off_flag(fighter.battle_object, elight::instance::flag::SPECIAL_S_CANCEL);
                }
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        elight_frame
    );
}