use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*, table_const::*}
};

#[line("shulk", main)]
fn shulk_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
        // Reset Vars

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
            VarModule::off_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_LW);
            VarModule::set_float(fighter.battle_object, shulk::instance::float::BURST_COOLDOWN, 0.0);
        }

        // Damage Check

        if !fighter.global_table[IS_STOP].get_bool()
        && !MiscModule::is_illegal_status(fighter.module_accessor, false)
        && !MiscModule::is_illegal_status(fighter.module_accessor, true)
        && MiscModule::is_damage_check(fighter.module_accessor, false)
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_DAMAGE_FALL
        && fighter.global_table[CMD_CAT1].get_i32() == *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW
        && !VarModule::is_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_LW) {
            let burst_recover = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);
            DamageModule::add_damage(fighter.module_accessor, burst_recover * -0.5, 0);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT, true);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_ATTACK_POWER);
            let target_id = VarModule::get_int(fighter.battle_object, fighter::instance::int::TARGET_ID) as u32;
            if sv_battle_object::is_active(target_id) {
                let target_boma = sv_battle_object::module_accessor(target_id);
                let shulkpos = PostureModule::pos_x(fighter.module_accessor);
                let opppos = PostureModule::pos_x(target_boma);
                if (shulkpos > opppos && PostureModule::lr(fighter.module_accessor) == 1.0)
                || (shulkpos < opppos && PostureModule::lr(fighter.module_accessor) == -1.0) {
                    PostureModule::reverse_lr(fighter.module_accessor);
                }
                SlowModule::set(target_boma, 0, 50, 60, false, *BATTLE_OBJECT_ID_INVALID as u32);
                VarModule::set_int(fighter.battle_object, fighter::instance::int::TARGET_ID, 0);
            }
            EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_sp_flash"), Hash40::new("head"), &Vector3f { x: -3.0, y: 3.0, z: 0.0 }, &Vector3f { x: -3.0, y: 3.0, z: 0.0 }, 1.0, &Vector3f { x: -3.0, y: 3.0, z: 0.0 }, &Vector3f { x: -3.0, y: 3.0, z: 0.0 }, false, 0, 0, 0);
            KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_RESET);
            KineticModule::unable_energy_all(fighter.module_accessor);
            VarModule::on_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_LW);
            VarModule::set_float(fighter.battle_object, shulk::instance::float::BURST_COOLDOWN, 3600.0);
        }

        // Special Lw Check
        let mut burst_cooldown = VarModule::get_float(fighter.battle_object, shulk::instance::float::BURST_COOLDOWN);
        if burst_cooldown > 0.0 {
            VarModule::sub_float(fighter.battle_object, shulk::instance::float::BURST_COOLDOWN, 1.0);
            burst_cooldown = VarModule::get_float(fighter.battle_object, shulk::instance::float::BURST_COOLDOWN);
            if burst_cooldown <= 0.0 {
                let pos: Vector3f = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                let rot: Vector3f = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                let countereff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_counter_flash"), Hash40::new("top"), &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                EffectModule::set_rgb(fighter.module_accessor, countereff, 0.0, 5.0, 5.0);
            }
        }
        else{
            VarModule::off_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_LW);
        }
    }
}

pub fn install() {
    shulk_frame::install();
}
