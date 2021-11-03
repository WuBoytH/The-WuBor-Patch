use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_WOLF )]
fn wolf_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if [
            hash40("special_s_end"),
            hash40("special_air_s_end")
        ].contains(&MotionModule::motion_kind(fighter.module_accessor)) {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                FLASH_CANCEL[entry_id(fighter.module_accessor)] = true;
            }
        }
        else {
            FLASH_CANCEL[entry_id(fighter.module_accessor)] = false;
        }

        if FLASH_CANCEL[entry_id(fighter.module_accessor)]
        && MotionModule::frame(fighter.module_accessor) >= 18.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        wolf_frame
    );
}