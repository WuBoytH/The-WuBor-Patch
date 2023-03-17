use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smashline::*
};

#[fighter_frame( agent = FIGHTER_KIND_BAYONETTA, main )]
fn bayonetta_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME);
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_100, false);
            }
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_s_u") {
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f{x: 9.0, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
        }
    }
}

pub fn install() {
    install_agent_frames!(
        bayonetta_frame
    );
}