use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*
};

#[fighter_frame( agent = FIGHTER_KIND_SZEROSUIT )]
fn szerosuit_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            && MotionModule::frame(fighter.module_accessor) > 30.0 {
                WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME)
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        szerosuit_frame
    );
}