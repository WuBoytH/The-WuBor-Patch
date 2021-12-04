use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::vars::*
};

#[fighter_frame( agent = FIGHTER_KIND_GEKKOUGA )]
fn gekkouga_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if [
            hash40("appeal_lw_l"),
            hash40("appeal_lw_r")
        ].contains(&MotionModule::motion_kind(fighter.module_accessor))
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && MotionModule::frame(fighter.module_accessor) > WorkModule::get_float(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLOAT_HIT_FRAME) + 1.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        gekkouga_frame
    );
}