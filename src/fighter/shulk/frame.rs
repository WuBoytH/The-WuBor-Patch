use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_SHULK )]
fn shulk_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
        // Reset Vars

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_LW);
            _TIME_COUNTER[entry_id(fighter.module_accessor)] = -1;
        }

        // Damage Check

        let damage = DamageModule::damage(fighter.module_accessor, 0);
        if damage > WorkModule::get_float(fighter.module_accessor, FIGHTER_SHULK_INSTANCE_WORK_ID_FLOAT_PREV_DAMAGE) {
            let burst_recover = damage - WorkModule::get_float(fighter.module_accessor, FIGHTER_SHULK_INSTANCE_WORK_ID_FLOAT_PREV_DAMAGE);
            WorkModule::set_float(fighter.module_accessor, burst_recover, FIGHTER_SHULK_INSTANCE_WORK_ID_FLOAT_BURST_RECOVER);
        }
        WorkModule::set_float(fighter.module_accessor, damage, FIGHTER_SHULK_INSTANCE_WORK_ID_FLOAT_PREV_DAMAGE);

        if StopModule::is_damage(fighter.module_accessor)
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_LW) {
            let burst_recover = WorkModule::get_float(fighter.module_accessor, FIGHTER_SHULK_INSTANCE_WORK_ID_FLOAT_BURST_RECOVER);
            DamageModule::add_damage(fighter.module_accessor, burst_recover * -0.5, 0);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT, true);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_ATTACK_POWER);
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_SHULK_MONAD_TYPE_DEFAULT, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE);
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_SHULK_MONAD_TYPE_DEFAULT, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE_SELECT);
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_SHULK_MONAD_TYPE_DEFAULT, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_EFFECT_HANDLE);
            let target_id = WorkModule::get_int64(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID) as u32;
            if sv_battle_object::is_active(target_id) {
                let target_boma = sv_battle_object::module_accessor(target_id);
                let shulkpos = PostureModule::pos_x(fighter.module_accessor);
                let opppos = PostureModule::pos_x(target_boma);
                if shulkpos > opppos && PostureModule::lr(fighter.module_accessor) == 1.0 {
                    PostureModule::reverse_lr(fighter.module_accessor);
                }
                else if shulkpos < opppos && PostureModule::lr(fighter.module_accessor) == -1.0 {
                    PostureModule::reverse_lr(fighter.module_accessor);
                }
                WorkModule::set_int64(fighter.module_accessor, 0, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID);
            }
            macros::SLOW_OPPONENT(fighter, 20.0, 60.0);
            EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_sp_flash"), Hash40::new("head"), &Vector3f { x: -3.0, y: 3.0, z: 0.0 }, &Vector3f { x: -3.0, y: 3.0, z: 0.0 }, 1.0, &Vector3f { x: -3.0, y: 3.0, z: 0.0 }, &Vector3f { x: -3.0, y: 3.0, z: 0.0 }, false, 0, 0, 0);
            KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_RESET);
            KineticModule::unable_energy_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_LW);
            _TIME_COUNTER[entry_id(fighter.module_accessor)] = 3600;
        }

        // Special Lw Check

        if _TIME_COUNTER[entry_id(fighter.module_accessor)] > 0 {
            _TIME_COUNTER[entry_id(fighter.module_accessor)] = _TIME_COUNTER[entry_id(fighter.module_accessor)] - 1;
            if _TIME_COUNTER[entry_id(fighter.module_accessor)] == 0 {
                _TIME_COUNTER[entry_id(fighter.module_accessor)] = -1;
                let pos: Vector3f = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                let rot: Vector3f = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                let countereff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_counter_flash"), Hash40::new("top"), &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                EffectModule::set_rgb(fighter.module_accessor, countereff, 0.0, 5.0, 5.0);
            }
        }
        else{
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_LW);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        shulk_frame
    );
}
