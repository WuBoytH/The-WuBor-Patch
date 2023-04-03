use crate::imports::status_imports::*;

unsafe extern "C" fn rockman_leaf_shield_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    if status == *FIGHTER_STATUS_KIND_REBIRTH {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_REBIRTH_FLAG_MOVE_END) {
            return 0.into();
        }
    }
    // if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD) {
    //     if WorkModule::is_enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)
    //     && !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD) {
    //         WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    //     }
    // }
    // else {
    //     let group = [
    //         // *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK,
    //         // *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK,
    //         *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE,
    //         *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE,
    //         // *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF,
    //     ];
    //     let terms = [
    //         // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON,
    //         // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD,
    //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
    //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
    //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B,
    //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH,
    //         // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
    //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
    //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
    //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
    //         // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT,
    //         // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY,
    //         // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
    //         // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH,
    //         // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN,
    //         *FIGHTER_STATUS_TRANSITION_TERM_ID_RUN,
    //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN,
    //         // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
    //         // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
    //         // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH,
    //         // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH,
    //         // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD,
    //         // *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH
    //     ];
    //     for x in group.iter() {
    //         WorkModule::unable_transition_term_group(fighter.module_accessor, *x);
    //     }
    //     for x in terms.iter() {
    //         WorkModule::unable_transition_term(fighter.module_accessor, *x);
    //     }
    //     let status = StatusModule::status_kind(fighter.module_accessor);
    //     if status == *FIGHTER_STATUS_KIND_DASH {
    //         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4);
    //         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_ESCAPE);
    //     }
    //     if status == *FIGHTER_STATUS_KIND_TURN {
    //         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_FLAG_NO_TURN_TO_ESCAPE);
    //     }
    //     if status == *FIGHTER_STATUS_KIND_RUN_BRAKE {
    //         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_RUN_BRAKE_FLAG_ATTACK_DASH_STRANS_OFF);
    //     }
    // }
    0.into()
}

unsafe extern "C" fn rockman_special_lw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    (!WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD)).into()
}

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_ROCKMAN {
            return;
        }
        fighter.global_table[CHECK_GROUND_SPECIAL_UNIQ].assign(&L2CValue::Ptr(rockman_leaf_shield_check as *const () as _));
        fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&L2CValue::Ptr(rockman_leaf_shield_check as *const () as _));
        fighter.global_table[DASH_COMMON_UNIQ].assign(&L2CValue::Ptr(rockman_leaf_shield_check as *const () as _));
        fighter.global_table[RUN_MAIN_UNIQ].assign(&L2CValue::Ptr(rockman_leaf_shield_check as *const () as _));
        fighter.global_table[JUMP_SQUAT_MAIN_UNIQ].assign(&L2CValue::Ptr(rockman_leaf_shield_check as *const () as _));
        fighter.global_table[GUARD_CONT_UNIQ].assign(&L2CValue::Ptr(rockman_leaf_shield_check as *const () as _));
        fighter.global_table[TURN_UNIQ].assign(&L2CValue::Ptr(rockman_leaf_shield_check as *const () as _));
        fighter.global_table[FALL_BRAKE_UNIQ].assign(&L2CValue::Ptr(rockman_leaf_shield_check as *const () as _));
        fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(rockman_special_lw_uniq as *const () as _));
    }
}

pub fn install() {
    install_agent_resets!(
        agent_reset
    );
}
