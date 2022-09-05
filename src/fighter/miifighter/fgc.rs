use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*, table_const::*}
};

pub unsafe extern "C" fn miifighter_fgc(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
        MiscModule::set_hp(fighter, 188.0);
        let status = fighter.global_table[STATUS_KIND].get_i32();
        let mut ground_normal = true;
        let mut normal_cancels = [].to_vec();
        let mut special_cancels = [].to_vec();
        let mut aerial_cancel = false;
        let mut jump_cancel = 0;
        if status == *FIGHTER_STATUS_KIND_ATTACK
        || status == *FIGHTER_STATUS_KIND_ITEM_SWING {
            FGCModule::disable_ground_normal(fighter, ATTACK_N_MASK);
        }
        else if status == *FIGHTER_STATUS_KIND_ATTACK_S3
        || status == *FIGHTER_STATUS_KIND_ITEM_SWING_S3 {
            FGCModule::disable_ground_normal(fighter, ATTACK_S3_MASK);
        }
        else if status == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
            FGCModule::disable_ground_normal(fighter, ATTACK_HI3_MASK);
        }
        else if status == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
            FGCModule::disable_ground_normal(fighter, ATTACK_LW3_MASK);
        }
        else if status == *FIGHTER_STATUS_KIND_ATTACK_S4_START
        || status == *FIGHTER_STATUS_KIND_ATTACK_S4
        || status == *FIGHTER_STATUS_KIND_ITEM_SWING_S4_START {
            FGCModule::disable_ground_normal(fighter, ATTACK_S4_MASK);
        }
        else if status == *FIGHTER_STATUS_KIND_ATTACK_HI4_START
        || status == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            FGCModule::disable_ground_normal(fighter, ATTACK_HI4_MASK);
            jump_cancel = 1;
        }
        else if status == *FIGHTER_STATUS_KIND_ATTACK_LW4_START
        || status == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
            FGCModule::disable_ground_normal(fighter, ATTACK_LW4_MASK);
        }
        else if status == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            // nothing haha
        }
        else {
            ground_normal = false;
            if status == *FIGHTER_STATUS_KIND_ATTACK_AIR {
                jump_cancel = 2;
                aerial_cancel = true;
                VarModule::on_flag(fighter.battle_object, commons::status::flag::NORMAL_CANCEL);
                let mot = MotionModule::motion_kind(fighter.module_accessor);
                let flags = ATTACK_AIR_N_MASK + ATTACK_AIR_F_MASK + ATTACK_AIR_B_MASK + ATTACK_AIR_HI_MASK + ATTACK_AIR_LW_MASK;
                VarModule::set_int(fighter.battle_object, commons::status::int::ENABLED_AERIALS, flags);
                if mot == hash40("attack_air_n") {
                    FGCModule::disable_aerial(fighter, ATTACK_AIR_N_MASK);
                }
                else if mot == hash40("attack_air_f") {
                    FGCModule::disable_aerial(fighter, ATTACK_AIR_F_MASK);
                }
                else if mot == hash40("attack_air_b") {
                    FGCModule::disable_aerial(fighter, ATTACK_AIR_B_MASK);
                }
                else if mot == hash40("attack_air_hi") {
                    FGCModule::disable_aerial(fighter, ATTACK_AIR_HI_MASK);
                }
                else {
                    FGCModule::disable_aerial(fighter, ATTACK_AIR_LW_MASK);
                }
                special_cancels = [
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
                ].to_vec();
            }
            else if status == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END {
                if MotionModule::frame(fighter.module_accessor) > 27.0 {
                    jump_cancel = 1;
                    special_cancels = [
                        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
                        *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
                    ].to_vec();
                }
            }
        }
        if ground_normal {
            normal_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START
            ].to_vec();
            special_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
            ].to_vec();
        }
        FGCModule::cancel_system(fighter, normal_cancels, special_cancels, aerial_cancel, jump_cancel);
    }
}
