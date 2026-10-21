use crate::imports::*;
use wubor_utils::offsets;
use smash_rs::app::{WorkId, work_ids, transition_groups, transition_terms};

extern "C" {
    #[link_name = "rockman_do_leafshield_things_disable_inline"]
    fn rockman_do_leafshield_things_disable_inline(ctx: &mut skyline::hooks::InlineCtx);

    #[link_name = "rockman_do_leafshield_things_enable_inline"]
    fn rockman_do_leafshield_things_enable_inline(ctx: &mut skyline::hooks::InlineCtx);
}

#[skyline::hook(offset = offsets::fighter::rockman::DO_LEAFSHIELD_THINGS_DISABLE, inline)]
unsafe extern "C" fn rockman_do_leafshield_things_disable(ctx: &mut skyline::hooks::InlineCtx) {
    rockman_do_leafshield_things_disable_inline(ctx);
}

#[skyline::hook(offset = offsets::fighter::rockman::DO_LEAFSHIELD_THINGS_ENABLE, inline)]
unsafe extern "C" fn rockman_do_leafshield_things_enable(ctx: &mut skyline::hooks::InlineCtx) {
    rockman_do_leafshield_things_enable_inline(ctx);
}

const LEAFSHIELD_DISABLE_GROUPS: [WorkId; 6] = [
    // transition_groups::CHECK_GROUND_SPECIAL,
    // transition_groups::CHECK_AIR_SPECIAL,
    transition_groups::CHECK_GROUND_ESCAPE,
    // transition_groups::CHECK_AIR_ESCAPE,
    transition_groups::CHECK_GROUND_GUARD,
    transition_groups::CHECK_GROUND_ATTACK,
    transition_groups::CHECK_GROUND_CATCH,
    transition_groups::CHECK_AIR_ATTACK,
    transition_groups::CHECK_AIR_CLIFF
];

const LEAFSHIELD_DISABLE_INDIVI: [WorkId; 8] = [
    // transition_terms::CONT_DASH,
    // transition_terms::CONT_TURN_DASH,
    transition_terms::CONT_ATTACK_DASH,
    transition_terms::CONT_CATCH_DASH,
    transition_terms::CONT_CATCH_TURN,
    // transition_terms::CONT_ATTACK_HI4_START,
    // transition_terms::CONT_ATTACK_LW4_START,
    transition_terms::CONT_SPECIAL_N,
    transition_terms::CONT_SPECIAL_S,
    transition_terms::CONT_SPECIAL_HI,
    transition_terms::CONT_CLIFF_ATTACK,
    transition_terms::CONT_CLIFF_ESCAPE
];

#[skyline::hook(replace = FighterSpecializer_Rockman::set_leafshield)]
unsafe extern "C" fn set_leafshield(module_accessor: *mut smash_rs::app::BattleObjectModuleAccessor, set_shield: bool) {
    let work = (*module_accessor).work();
    work.set_flag(set_shield, work_ids::fighter::rockman::instance::SPECIAL_LW_LEAFSHIELD);
    work.set_flag(set_shield, work_ids::fighter::rockman::instance::SPECIAL_LW_ENABLE_SHOOT);
    if !set_shield {
        work.set_int(0, work_ids::fighter::rockman::instance::SPECIAL_LW_HOLD_FRAME);
        for x in LEAFSHIELD_DISABLE_GROUPS.iter() {
            work.unable_transition_term_forbid_group_indivi(*x);
        }
        for x in LEAFSHIELD_DISABLE_INDIVI.iter() {
            work.unable_transition_term_forbid_indivi(*x);
        }
        // work.enable_transition_term_forbid(transition_terms::CONT_SPECIAL_LW);
        // if (*module_accessor).status().status_kind() < 0x27 {
        //     for x in LEAFSHIELD_DISABLE_GROUPS.iter() {
        //         work.enable_transition_term_group(*x);
        //     }
        //     for x in LEAFSHIELD_DISABLE_INDIVI.iter() {
        //         work.enable_transition_term(*x);
        //     }
        // }
    }
    else {
        let hold_frame = work.get_param_int(smash_rs::phx::Hash40::new("param_special_lw"), smash_rs::phx::Hash40::new("hold_frame"));
        work.set_int(hold_frame, work_ids::fighter::rockman::instance::SPECIAL_LW_HOLD_FRAME);
        for x in LEAFSHIELD_DISABLE_GROUPS.iter() {
            work.enable_transition_term_forbid_group_indivi(*x);
        }
        for x in LEAFSHIELD_DISABLE_INDIVI.iter() {
            work.enable_transition_term_forbid_indivi(*x);
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        rockman_do_leafshield_things_disable,
        rockman_do_leafshield_things_enable,
        set_leafshield
    );
}
