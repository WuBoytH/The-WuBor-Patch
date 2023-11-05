use {
    smash::{
        lua2cpp::L2CAgentBase,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    wubor_utils::vars::*
};

pub struct UnstanceGame {
    pub hit_effect: u64,
    pub hit_sound: i32
}

pub struct UnstanceEffect {
    pub trail1: u64,
    pub trail2: u64,
    pub swordflare: u64
}

pub unsafe fn marth_unstance_game(fighter: &mut L2CAgentBase, mut hit_effect: u64, mut hit_sound: i32) -> UnstanceGame {
    if marth_is_unstance(fighter) {
        hit_effect = hash40("collision_attr_fire");
        hit_sound = *COLLISION_SOUND_ATTR_FIRE;
    }
    UnstanceGame{hit_effect, hit_sound}
}

pub unsafe fn marth_unstance_effect(fighter: &mut L2CAgentBase, mut trail1: u64, mut trail2: u64, mut swordflare: u64) -> UnstanceEffect {
    if marth_is_unstance(fighter) {
        trail1 = hash40("tex_marth_unstance_sword1");
        trail2 = hash40("tex_marth_unstance_sword2");
        swordflare = hash40("marth_sword_red");
    }
    UnstanceEffect{trail1, trail2, swordflare}
}

pub unsafe fn marth_is_unstance(fighter: &mut L2CAgentBase) -> bool {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut prev_status = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    if [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4
    ].contains(&status) {
        if [
            *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD
        ].contains(&prev_status) {
            prev_status = StatusModule::prev_status_kind(fighter.module_accessor, 2);
        }
        else {
            prev_status = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        }
    }
    else if status == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP {
        prev_status = StatusModule::prev_status_kind(fighter.module_accessor, 1);
    }
    else if [
        *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX
    ].contains(&status) {
        if prev_status == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP {
            prev_status = StatusModule::prev_status_kind(fighter.module_accessor, 2);
        }
        else {
            prev_status = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        }
    }
    let range_lw = marth::status::STANCE_DASH_F;
    let range_hi = marth::status::STANCE_SPECIAL_S;
    let stance_range = (range_lw..range_hi).contains(&prev_status);
    prev_status == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT || stance_range
}
