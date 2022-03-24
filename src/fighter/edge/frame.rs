use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    wubor_utils::vars::*
};

#[fighter_frame( agent = FIGHTER_KIND_EDGE )]
fn edge_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Cancel Frames

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCEL);
        }
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCEL);
            }
        }
        if (StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING)
        && WorkModule::is_flag(fighter.module_accessor, FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCEL) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        edge_frame
    );
}