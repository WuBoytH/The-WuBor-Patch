use crate::imports::status_imports::*;
use smash_rs::app::{WorkId, work_ids, transition_groups, transition_terms};

#[skyline::hook(offset = 0x107e950)]
pub unsafe extern "C" fn rockman_vtable_func(vtable: u64, fighter: &mut smash::app::Fighter) {
    let module_accessor = (fighter.battle_object).module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    if (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0)
    && !StopModule::is_stop(module_accessor) && !SlowModule::is_skip(module_accessor) {
        if ![
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_LW_SHOOT
        ].contains(&status)
        && WorkModule::is_flag(module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD) {
            WorkModule::dec_int(module_accessor, 0x100000c3); // FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_SPECIAL_LW_HOLD_FRAME
            if WorkModule::is_flag(module_accessor, 0x200000e1) { // FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_ENABLE_SHOOT
                if WorkModule::get_int(module_accessor, 0x100000c3) <= 0 {
                    LinkModule::send_event_nodes(
                        module_accessor,
                        *LINK_NO_ARTICLE,
                        Hash40::new_raw(0x2435e7c874),
                        0
                    );
                    ArticleModule::remove_exist(module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                    FighterSpecializer_Rockman::set_leafshield(module_accessor, false);
                }
                else if ControlModule::get_button(module_accessor) >> 1 & 1 != 0 {
                    StatusModule::change_status_request(module_accessor, *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_LW_SHOOT, false);
                }
            }
        }
    }
    let leafshield = WorkModule::is_flag(module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD);
    WorkModule::off_flag(module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD);
    original!()(vtable, fighter);
    WorkModule::set_flag(module_accessor, leafshield, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD);
}

// const ROCKMAN_DISABLE_GROUPS: [WorkId; 4] = [
//     transition_groups::CHECK_GROUND_SPECIAL,
//     transition_groups::CHECK_AIR_SPECIAL,
//     transition_groups::CHECK_GROUND_ESCAPE,
//     transition_groups::CHECK_AIR_ESCAPE,
// ];

// const ROCKMAN_DISABLE_INDIVI: [WorkId; 2] = [
//     transition_terms::CONT_ATTACK_DASH,
//     transition_terms::CONT_LADDER_ATTACK
// ];

// #[skyline::hook(replace = FighterSpecializer_Rockman::set_leafshield)]
// unsafe extern "C" fn set_leafshield(module_accessor: *mut smash_rs::app::BattleObjectModuleAccessor, set_shield: bool) {
//     // let groups = [
//     //     *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL,
//     //     // *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM,
//     //     // *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH,
//     //     // *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK,
//     //     *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE,
//     //     // *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD,
//     //     // *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK,
//     //     *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE
//     // ];
//     // let individual = [
//     //     *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH
//     // ];
//     let work = (*module_accessor).work();
//     work.set_flag(set_shield, work_ids::fighter::rockman::instance::SPECIAL_LW_LEAFSHIELD);
//     work.set_flag(set_shield, work_ids::fighter::rockman::instance::SPECIAL_LW_ENABLE_SHOOT);
//     if !set_shield {
//         for x in ROCKMAN_DISABLE_GROUPS.iter() {
//             work.unable_transition_term_forbid_group_indivi(*x);
//         }
//         for x in ROCKMAN_DISABLE_INDIVI.iter() {
//             work.unable_transition_term_forbid_indivi(*x);
//         }
//         // WorkModule::enable_transition_term_forbid(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
//         // if StatusModule::status_kind(module_accessor) < 0x27 {
//         //     for x in groups.iter() {
//         //         WorkModule::enable_transition_term_group(module_accessor, *x);
//         //     }
//         // }
//     }
//     else {
//         for x in ROCKMAN_DISABLE_GROUPS.iter() {
//             work.enable_transition_term_forbid_group_indivi( *x);
//         }
//         for x in ROCKMAN_DISABLE_INDIVI.iter() {
//             work.enable_transition_term_forbid_indivi(*x);
//         }
//     }
// }

pub fn install() {
    skyline::install_hooks!(
        rockman_vtable_func,
        // set_leafshield
    );
}