use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::*,
        app::*,
        lib::lua_const::*
    },
    smashline::*,
    custom_cancel::*
};

#[event(start)]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_WOLF {
            return;
        }
        let agent = Hash40::new("fighter_kind_wolf");
        CustomCancelManager::add_cancel_info(
            agent,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            CancelInfo::new()
                .enable_normals([
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
                ].to_vec())
                .enable_jump_cancel(CancelType::HIT)
        );
    }
}

pub fn install() {
    let agent = Hash40::new("fighter_kind_wolf");
    CustomCancelManager::initialize_agent(agent);
    agent_reset::install();
}
