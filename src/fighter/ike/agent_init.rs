use {
    smash::{
        lua2cpp::L2CFighterCommon,
        // phx::*,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    // custom_cancel::*,
    wubor_utils::{vars::*, table_const::*},
    crate::fighter::common::agent_inits::*
};

pub unsafe extern "C" fn ike_status_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
    || status == *FIGHTER_STATUS_KIND_REBIRTH {
        VarModule::off_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_S);
    }
    0.into()
}

#[event(start)]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_IKE {
            return;
        }
        fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(specials_pre_generic as *const () as _));
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(ike_status_end_control as *const () as _));
        // let agent = Hash40::new("fighter_kind_ike");
        // CustomCancelManager::initialize_agent(agent);
        // generic_attack(agent);
        // generic_attackair(agent);
        // CustomCancelManager::add_cancel_info(
        //     agent,
        //     *FIGHTER_STATUS_KIND_ATTACK_HI3,
        //     CancelInfo::new()
        //         .enable_normals([
        //             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
        //             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
        //             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
        //         ].to_vec())
        //         .enable_jump_cancel(CancelType::HIT)
        //         .jump_cancel_require_flag()
        //         .set_fgc_flags(FGCFlags::ALL - FGCFlags::JUMP)
        // );
        // generic_attack3(agent);
        // generic_attack4(agent);
    }
}

pub fn install() {
    // let agent = Hash40::new("fighter_kind_ike");
    // CustomCancelManager::initialize_agent(agent);
    agent_reset::install();
}
