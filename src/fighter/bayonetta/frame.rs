use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*
};

#[fighter_frame( agent = FIGHTER_KIND_BAYONETTA )]
fn bayonetta_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_100, false);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        bayonetta_frame
    );
}