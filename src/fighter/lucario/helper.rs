use {
    smash::{
        lua2cpp::*,
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::{vars::*, table_const::*},
    super::vl
};

pub unsafe extern "C" fn lucario_drain_aura(fighter: &mut L2CAgentBase, drain_all: bool) -> bool {
    let aura_charge = VarModule::get_int(fighter.module_accessor, lucario::instance::int::AURA_LEVEL);
    if aura_charge > 0 {
        if drain_all {
            VarModule::set_int(fighter.module_accessor, lucario::instance::int::AURA_LEVEL, 0);
            VarModule::set_int(fighter.module_accessor, lucario::status::int::AURA_ENHANCED_BY, aura_charge);
        }
        else {
            VarModule::dec_int(fighter.module_accessor, lucario::instance::int::AURA_LEVEL);
            VarModule::inc_int(fighter.module_accessor, lucario::status::int::AURA_ENHANCED_BY);
        }
        true
    }
    else {
        false
    }
}

pub unsafe extern "C" fn lucario_gain_aura(fighter: &mut L2CAgentBase) -> bool {
    if VarModule::get_int(fighter.module_accessor, lucario::instance::int::AURA_LEVEL) < vl::private::AURA_CHARGE_MAX {
        VarModule::inc_int(fighter.module_accessor, lucario::instance::int::AURA_LEVEL);
        FighterUtil::flash_eye_info(fighter.module_accessor);
        true
    }
    else {
        false
    }
}

pub unsafe extern "C" fn lucario_special_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        lucario_special_set_air(fighter);
        lucario_special_air_mot_helper(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        lucario_special_set_ground(fighter);
        lucario_special_ground_mot_helper(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
}

pub unsafe extern "C" fn lucario_special_set_ground(fighter: &mut L2CFighterCommon) {
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
}

pub unsafe extern "C" fn lucario_special_set_air(fighter: &mut L2CFighterCommon) {
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
}

pub unsafe extern "C" fn lucario_special_ground_mot_helper(fighter: &mut L2CFighterCommon) {
    let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
        MotionModule::change_motion_inherit_frame_keep_rate(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            -1.0,
            1.0,
            0.0
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    }
}

pub unsafe extern "C" fn lucario_special_air_mot_helper(fighter: &mut L2CFighterCommon) {
    let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
        MotionModule::change_motion_inherit_frame_keep_rate(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            -1.0,
            1.0,
            0.0
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    }
}

pub unsafe extern "C" fn lucario_special_n_joint_translate(fighter: &mut L2CFighterCommon) {
    let havel = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let haver = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_position(
        fighter.module_accessor,
        Hash40::new("havel"),
        havel,
        true
    );
    ModelModule::joint_global_position(
        fighter.module_accessor,
        Hash40::new("haver"),
        haver,
        true
    );
    let new_pos = Vector3f{x: havel.x + haver.x, y: havel.y + haver.y, z: havel.z + haver.z};
    let new_pos = Vector3f{x: new_pos.x * 0.5, y: new_pos.y * 0.5, z: new_pos.z * 0.5};
    ModelModule::set_joint_translate(
        fighter.module_accessor,
        Hash40::new("throw"),
        &new_pos,
        true,
        false
    );
}

pub unsafe extern "C" fn lucario_special_n_save_charge_status(fighter: &mut L2CFighterCommon) {
    let kind = fighter.global_table[KIND].get_i32();
    let status = fighter.global_table[STATUS_KIND].get_i32();
    let statuses = if kind != *FIGHTER_KIND_KIRBY {
        [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL
        ]
    }
    else {
        [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_HOLD,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_MAX,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_SHOOT,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_CANCEL
        ]
    };
    if !statuses.contains(&status) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    // lucario_special_n_save_charge_status_shoot(fighter);
}

// unsafe extern "C" fn lucario_special_n_save_charge_status_shoot(fighter: &mut L2CFighterCommon) {
//     let kind = fighter.global_table[FIGHTER_KIND].get_i32();
//     let status = StatusModule::status_kind(fighter.module_accessor);
//     let status_global = fighter.global_table[STATUS_KIND].get_i32();
//     let statuses = if kind != *FIGHTER_KIND_KIRBY {
//         [
//             *FIGHTER_STATUS_KIND_SPECIAL_N,
//             *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
//             *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
//             *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
//             *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL,
//             *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL
//         ]
//     }
//     else {
//         [
//             *FIGHTER_STATUS_KIND_SPECIAL_N,
//             *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_HOLD,
//             *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_MAX,
//             *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_SHOOT,
//             *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_CANCEL,
//             *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL
//         ]
//     };
//     if status == statuses[0] {
//         if status_global == statuses[3] {
//             EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
//         }
//     }
//     if status == statuses[3] {
//         FighterSpecializer_Lucario::save_aura_ball_status(fighter.module_accessor, false, 0);
//     }
// }

pub unsafe extern "C" fn lucario_special_lw_eff_remover(fighter: &mut L2CAgentBase) {
    for x in lucario::status::int::SPECIAL_LW_EFF1..=lucario::status::int::SPECIAL_LW_EFF3 {
        let eff = VarModule::get_int(fighter.module_accessor, x) as u32;
        if EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            EffectModule::kill(fighter.module_accessor, eff, true, true);
        }
    }
}
