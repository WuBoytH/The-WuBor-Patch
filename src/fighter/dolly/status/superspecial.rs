use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_superspecial_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, dolly::instance::flag::IS_SPECIAL_CANCEL);
    dolly_superspecial_main_helper(fighter, hash40("param_super_special").into());
    let eff_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_INT_SCREEN_EFFECT_COUNT);
    if 0 < eff_count {
        MotionAnimcmdModule::call_script_single(
            fighter.module_accessor,
            *FIGHTER_ANIMCMD_EFFECT,
            Hash40::new("effect_superspecialcancelfillscreen"),
            -1
        );
    }
    0.into()
}

unsafe extern "C" fn dolly_superspecial_main_helper(fighter: &mut L2CFighterCommon, hash: L2CValue) {
    let param = hash.get_u64();
    let map_coll_joint = WorkModule::get_param_int64(fighter.module_accessor, param, hash40("map_coll_joint"));
    let offx = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_FLOAT_MAP_COLL_OFFSET_X);
    let offy = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_FLOAT_MAP_COLL_OFFSET_Y);
    let offz = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_FLOAT_MAP_COLL_OFFSET_Z);
    GroundModule::set_shape_data_rhombus_modify_node_offset(fighter.module_accessor, Hash40::new_raw(map_coll_joint), &Vector3f{x: offx, y: offy, z: offz});
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_superspecial2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW {
        VarModule::off_flag(fighter.battle_object, dolly::instance::flag::IS_SPECIAL_CANCEL);
        ArticleModule::remove_exist(
            fighter.module_accessor,
            *FIGHTER_DOLLY_GENERATE_ARTICLE_FIRE,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)
        );
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("dolly_buster_punch"), true, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("dolly_buster_dash"), true, true);
    }
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_superspecial2_blow_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_STATUS_KIND_WAIT
    && status != *FIGHTER_STATUS_KIND_FALL
    && status != *FIGHTER_STATUS_KIND_CATCH_CUT {
        CatchModule::catch_cut(fighter.module_accessor, false, false);
    }
    if ![
        *FIGHTER_STATUS_KIND_ATTACK_DASH
    ].contains(&status) {
        VarModule::off_flag(fighter.battle_object, dolly::instance::flag::RISING_FORCE);
        EffectModule::clear_screen(fighter.module_accessor, 1, 0x14);
    }
    else {
        VarModule::on_flag(fighter.battle_object, dolly::instance::flag::RISING_FORCE);
    }
    VarModule::off_flag(fighter.battle_object, dolly::instance::flag::IS_SPECIAL_CANCEL);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        dolly_superspecial_end,
        dolly_superspecial2_end,
        dolly_superspecial2_blow_end
    );
}