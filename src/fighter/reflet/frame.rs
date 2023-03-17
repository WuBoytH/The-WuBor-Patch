use {
    smash::{
        lua2cpp::*,
        hash40,
        app::{lua_bind::*, smashball},
        lib::lua_const::*
    },
    smashline::*,
    wubor_utils::wua_bind::*
};

#[fighter_frame( agent = FIGHTER_KIND_REFLET, main )]
fn reflet_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if smashball::is_training_mode() {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            && ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                WorkModule::set_int(fighter.module_accessor, 8, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            }
        }

        if (MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi")
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_hi"))
        && MotionModule::frame(fighter.module_accessor) >= 12.0 && MotionModule::frame(fighter.module_accessor) < 46.0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            || WarkModule::is_operation_cpu(fighter.module_accessor) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        reflet_frame
    );
}