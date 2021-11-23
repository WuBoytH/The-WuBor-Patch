use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::vars::*
};

#[fighter_frame( agent = FIGHTER_KIND_ROCKMAN )]
fn rockman_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY)
        && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW3
        && MotionModule::frame(fighter.module_accessor) > 8.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        rockman_frame
    );
}