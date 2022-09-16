use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::lua_const::*
    },
    smashline::*,
    custom_cancel::*,
    crate::fighter::common::common_fgc::*
};

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_WOLF {
            return;
        }
        let agent = (*fighter.battle_object).agent_kind_hash;
        generic_attack(agent);
        generic_attackair(agent);
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
                .set_fgc_flags(FGCFlags::NORMAL | FGCFlags::SPECIAL | FGCFlags::AERIAL | FGCFlags::DASH | FGCFlags::AIRDASH)
        );
        generic_attack3(agent);
        generic_attack4(agent);
    }
}

pub fn install() {
    install_agent_resets!(
        agent_reset
    );
}
