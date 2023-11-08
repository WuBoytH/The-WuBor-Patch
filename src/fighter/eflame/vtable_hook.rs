use smash_rs::app::*;

// Original Groups
// const EFLAME_DISABLE_GROUPS: [WorkId; 6] = [
//     transition_groups::CHECK_GROUND_SPECIAL,
//     transition_groups::CHECK_GROUND_ITEM,
//     transition_groups::CHECK_GROUND_CATCH,
//     transition_groups::CHECK_GROUND_ATTACK,
//     transition_groups::CHECK_AIR_SPECIAL,
//     transition_groups::CHECK_AIR_ATTACK
// ];

// Original Individuals
// const EFLAME_DISABLE_INDIVI: [WorkId; 14] = [
//     transition_terms::CONT_APPEAL_U,
//     transition_terms::CONT_APPEAL_S,
//     transition_terms::CONT_APPEAL_LW,
//     transition_terms::CONT_ATTACK_DASH,
//     transition_terms::CONT_CATCH_DASH,
//     transition_terms::CONT_CATCH_TURN,
//     transition_terms::CONT_ITEM_THROW_GUARD,
//     transition_terms::CONT_ITEM_THROW_FORCE_DASH,
//     transition_terms::CONT_ITEM_THROW_DASH,
//     transition_terms::CONT_ITEM_SWING_DASH,
//     transition_terms::CONT_ITEM_THROW_RUN_BRAKE_HI4,
//     transition_terms::CONT_LADDER_ATTACK,
//     transition_terms::CONT_ITEM_PICKUP_LIGHT_DASH,
//     transition_terms::CONT_ITEM_PICKUP_HEAVY_DASH,
// ];

const EFLAME_DISABLE_GROUPS: [WorkId; 2] = [
    transition_groups::CHECK_GROUND_SPECIAL,
    transition_groups::CHECK_AIR_SPECIAL
];

const EFLAME_DISABLE_INDIVI: [WorkId; 10] = [
    transition_terms::CONT_ATTACK_S3,
    transition_terms::CONT_ATTACK_HI3,
    transition_terms::CONT_ATTACK_LW3,
    transition_terms::CONT_ATTACK_S4_START,
    transition_terms::CONT_ATTACK_HI4_START,
    transition_terms::CONT_ATTACK_LW4_START,
    transition_terms::CONT_THROW_LW,
    transition_terms::CONT_APPEAL_LW,
    transition_terms::CONT_ATTACK_DASH,
    transition_terms::CONT_LADDER_ATTACK
];

#[skyline::hook(offset = 0xa0c230)]
pub unsafe extern "C" fn eflame_has_esword_enable_transitions(work: &mut WorkModule, has_sword: u64) {
    if has_sword == 0 {
        work.on_flag(work_ids::fighter::eflame::instance::HAS_ESWORD);
        for id in EFLAME_DISABLE_GROUPS.iter() {
            work.unable_transition_term_forbid_group_indivi(*id);
        }
        for id in EFLAME_DISABLE_INDIVI.iter() {
            work.unable_transition_term_forbid_indivi(*id);
        }
        // Vanilla (Unused)
        // let no_pickup_item_frame = work.get_int(work_ids::fighter::instance::NO_PICKUP_ITEM_FRAME);
        // if no_pickup_item_frame == 0 {
        //     work.off_flag(work_ids::fighter::instance::NO_PICKUP_ITEM);
        // }
        // work.off_flag(work_ids::fighter::instance::NO_DROP_ITEM);
        // work.off_flag(work_ids::fighter::instance::DISABLE_AIR_GET_ITEM);
        // work.off_flag(work_ids::fighter::instance::NO_SPECIALFLAG_HOIST);
    }
    else {
        work.off_flag(work_ids::fighter::eflame::instance::HAS_ESWORD);
        for id in EFLAME_DISABLE_GROUPS.iter() {
            work.enable_transition_term_forbid_group_indivi(*id);
        }
        for id in EFLAME_DISABLE_INDIVI.iter() {
            work.enable_transition_term_forbid_indivi(*id);
        }
        // Vanilla (Unused)
        // work.on_flag(work_ids::fighter::instance::NO_PICKUP_ITEM);
        // work.on_flag(work_ids::fighter::instance::NO_DROP_ITEM);
        // work.on_flag(work_ids::fighter::instance::DISABLE_AIR_GET_ITEM);
        // work.on_flag(work_ids::fighter::instance::NO_SPECIALFLAG_HOIST);
    }
}

#[skyline::hook(offset = 0xa0c860)]
pub unsafe extern "C" fn eflame_has_esword_disable_statuses(_vtable: u64, _fighter: &mut smash::app::Fighter, _status: i32) -> bool {
    // Vanilla Logic (Unused)
    // let module_accessor = _fighter.battle_object.module_accessor;
    // let has_sword = WorkModule::get_flag(module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_HAS_ESWORD);
    // 2 < _status - 0x8d || has_sword
    true
}

pub fn install() {
    skyline::install_hooks!(
        eflame_has_esword_enable_transitions,
        eflame_has_esword_disable_statuses
    );
}