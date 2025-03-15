use super::*;
use super::super::passive::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_DamageFly)]
unsafe extern "C" fn status_pre_damagefly(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_flick_x(fighter.module_accessor);
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_trigger(fighter.module_accessor);
    if is_bad_passive(fighter).get_bool() {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL);
        return 1.into();
    }
    let mut attr = *FIGHTER_STATUS_ATTR_DAMAGE | *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY;
    let mut flag_keep = *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_FLAG;
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if [
        *FIGHTER_STATUS_KIND_BAYONETTA_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_DEDEDE_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_FALCO_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_FOX_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_GAOGAEN_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_KAMUI_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_KROOL_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_RIDLEY_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_ROCKMAN_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_SIMON_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_WARIO_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_YOSHI_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_SHEIK_FINAL_CAPTURE
    ].contains(&prev_status) {
        attr |= *FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT;
    }
    if prev_status != *FIGHTER_STATUS_KIND_THROWN {
        flag_keep = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG;
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_THROWN_WORK_FLAG_DISABLE_PASSIVE) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE);
        }
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_DAMAGE_FLY,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        flag_keep,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_FLOAT,
        0
    );
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_NO_DROP_ITEM) {
        attr |= *FIGHTER_STATUS_ATTR_NO_DROP_ITEM;
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_NO_DROP_ITEM);
    }
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        attr as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_update_damage_fly_effect)]
unsafe extern "C" fn sub_update_damage_fly_effect(
    fighter: &mut L2CFighterCommon,
    some_bool: L2CValue,
    effect_const: L2CValue,
    effect_hash: L2CValue,
    some_val: L2CValue,
    attacker_color: L2CValue,
    some_bool2: L2CValue,
    _reaction_frame: L2CValue
) -> L2CValue {
    let mut effect_id = WorkModule::get_int(fighter.module_accessor, effect_const.get_i32()) as u32;
    if !some_bool.get_bool() {
        if effect_id != *EFFECT_HANDLE_NULL as u32 {
            if effect_hash.get_u64() != hash40("sys_flyroll_smoke") {
                effect!(fighter, MA_MSC_EFFECT_REMOVE, effect_id, 0x12);
            }
            else {
                EffectModule::detach(fighter.module_accessor, effect_id, 0x12);
            }
            WorkModule::set_int(fighter.module_accessor, *EFFECT_HANDLE_NULL, effect_const.get_i32());
        }
        return EFFECT_HANDLE_NULL.into();
    }
    if effect_id != *EFFECT_HANDLE_NULL as u32 {
        if some_bool2.get_bool() {
            effect!(fighter, MA_MSC_EFFECT_REMOVE, effect_id, 0);
            effect_id = *EFFECT_HANDLE_NULL as u32;
        }
    }
    if effect_id == *EFFECT_HANDLE_NULL as u32 {
        let damage_fly_smoke_node_param;
        let damage_fly_smoke_offset_x_param;
        let damage_fly_smoke_offset_y_param;
        if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL {
            damage_fly_smoke_node_param = hash40("damage_fly_smoke_node");
            damage_fly_smoke_offset_x_param = hash40("damage_fly_smoke_offset_x");
            damage_fly_smoke_offset_y_param = hash40("damage_fly_smoke_offset_y");
        }
        else {
            damage_fly_smoke_node_param = hash40("damage_fly_roll_smoke_node");
            damage_fly_smoke_offset_x_param = hash40("damage_fly_roll_smoke_offset_x");
            damage_fly_smoke_offset_y_param = hash40("damage_fly_roll_smoke_offset_y");
        }
        let damage_fly_smoke_node = WorkModule::get_param_int64(fighter.module_accessor, damage_fly_smoke_node_param, 0);
        let damage_fly_smoke_offset_x = WorkModule::get_param_float(fighter.module_accessor, damage_fly_smoke_offset_x_param, 0);
        let damage_fly_smoke_offset_y = WorkModule::get_param_float(fighter.module_accessor, damage_fly_smoke_offset_y_param, 0);
        if effect_hash.get_u64() == hash40("sys_flyroll_smoke") {
            // <Original>
            // let damage_level3 = WorkModule::get_param_float(fighter.module_accessor, hash40("battle_object"), hash40("damage_level3"));
            // let something = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x28249022cf);
            // let ratio = ((reaction_frame.get_f32() - damage_level3) / (something - damage_level3)).clamp(0.0, 1.0);
            // let fly_effect_smoke_life_min_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("fly_effect_smoke_life_min_rate"));
            // let lifetime_rate = fighter.lerp(
            //     fly_effect_smoke_life_min_rate.into(),
            //     1.0_f32.into(),
            //     ratio.into()
            // ).get_f32();
            // EffectModule::preset_lifetime_rate_partial(fighter.module_accessor, lifetime_rate);
            // </Original>
            // <WuBor>
            let fly_effect_smoke_life_min_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("fly_effect_smoke_life_min_rate"));
            EffectModule::preset_lifetime_rate_partial(fighter.module_accessor, fly_effect_smoke_life_min_rate);
            // </WuBor>
        }

        // <WuBor>
        let size = if effect_hash.get_u64() == hash40("sys_flyroll_aura") {
            0.65
        }
        else {
            0.8
        };
        // </WuBor>
        effect_id = EffectModule::req_follow(
            fighter.module_accessor,
            Hash40::new_raw(effect_hash.get_u64()),
            Hash40::new_raw(damage_fly_smoke_node),
            &Vector3f{x: damage_fly_smoke_offset_x, y: damage_fly_smoke_offset_y, z: 0.0},
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            size,
            true,
            0,
            some_val.get_i32(),
            attacker_color.get_i32(),
            0,
            0,
            false,
            false
        ) as u32;
        if effect_hash.get_u64() == hash40("sys_flyroll_smoke") {
            EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        }
        if attacker_color.get_i32() != -1 {
            if effect_hash.get_u64() != hash40("sys_flyroll_aura") // 0x10edd30e6b
            && effect_hash.get_u64() != hash40("sys_flyroll_smoke") {
                WorkModule::set_int(fighter.module_accessor, effect_id as i32, effect_const.get_i32());
                return effect_id.into();
            }
            let color_hash = if effect_hash.get_u64() != hash40("sys_flyroll_smoke") {
                0x1991118526
            }
            else {
                0x1aa5a68c3b
            };
            let team_color = FighterUtil::get_effect_team_color(EColorKind(attacker_color.get_i32()), Hash40::new_raw(color_hash));
            EffectModule::set_rgb_partial_last(fighter.module_accessor, team_color.value[0], team_color.value[1], team_color.value[2]);
        }

        WorkModule::set_int(fighter.module_accessor, effect_id as i32, effect_const.get_i32());
    }

    if effect_hash.get_u64() == hash40("sys_flyroll_smoke") {
        // <WuBor>
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        let speed = sv_kinetic_energy::get_speed_length(fighter.lua_state_agent);
        let fly_effect_smoke_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("fly_effect_smoke_speed"));
        let fly_effect_aura_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("fly_effect_aura_speed"));
        let lerp = (speed.clamp(fly_effect_smoke_speed, fly_effect_aura_speed) - fly_effect_smoke_speed) / (fly_effect_aura_speed - fly_effect_smoke_speed);
        EffectModule::set_alpha(fighter.module_accessor, effect_id, 0.8 * lerp);
        // </WuBor>
    }

    effect_id.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_DamageFlyChkUniq)]
unsafe extern "C" fn sub_damageflychkuniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_ENABLE_DOWN) {
            if fighter.sub_AirChkDown().get_bool() {
                return true.into();
            }
        }
        else {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            let damage_speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            let damage_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_FRAME);
            if -1.0 <= damage_speed_y
            && WorkModule::get_param_int(fighter.module_accessor, hash40("common"), 0x1e7a52eb8a) <=damage_frame {
                if fighter.sub_AirChkDown().get_bool() {
                    return true.into();
                }
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_ENABLE_DOWN);
        }
        // if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER)
        // && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GENESISSET) {
        //     fighter.clear_lua_stack();
        //     lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        //     let damage_speed_length = sv_kinetic_energy::get_speed_length(fighter.lua_state_agent);
        //     if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_ATTACK_DISABLE_FRAME) <= 0 {
        //         if damage_speed_length <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("damage_fly_attack_speed")) {
        //             WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
        //             WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
        //         }
        //     }
        //     if 1.0 < fighter.global_table[MOTION_FRAME].get_f32()
        //     && WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_ESCAPE_DISABLE_FRAME) <= 0 {
        //         if damage_speed_length <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("damage_fly_escape_speed")) {
        //             WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LASSO);
        //             WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
        //         }
        //     }
        //     if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_DAMAGE_REFLECT_ESCAPE_DISABLE_FRAME) <= 0 {
        //         WorkModule::enable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
        //     }
        // }
        fighter.FighterStatusDamage__check_smoke_effect();
    }
    false.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_damagefly,
            sub_update_damage_fly_effect,
            sub_damageflychkuniq
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}