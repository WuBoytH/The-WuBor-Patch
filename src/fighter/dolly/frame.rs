use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        gameplay::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_DOLLY )]
fn dolly_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
            if DTILT_CHAIN[entry_id(fighter.module_accessor)] == 0 {
                chain_cancels(fighter, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3, true, &mut DTILT_CHAIN);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        dolly_frame
    );
}