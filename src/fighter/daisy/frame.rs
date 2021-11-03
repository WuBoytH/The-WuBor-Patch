use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_DAISY )]
fn daisy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR
        || is_damage_check(fighter.module_accessor, false) {
            DISABLE_SPECIAL_S[entry_id(fighter.module_accessor)] = false;
        }
    }
}

pub fn install() {
    install_agent_frames!(
        daisy_frame
    );
}