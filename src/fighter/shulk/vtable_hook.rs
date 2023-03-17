use crate::imports::status_imports::*;

#[skyline::hook(offset = 0x116a3b0)]
pub unsafe extern "C" fn shulk_check_valid_arts_statuses(fighter: *mut Fighter) -> u64 {
    let module_accessor = (*fighter).battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    u64::from([
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_TURN,
        *FIGHTER_STATUS_KIND_TURN_DASH,
        *FIGHTER_STATUS_KIND_TURN_RUN,
        *FIGHTER_STATUS_KIND_JUMP_SQUAT, // new
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_FLY,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
        *FIGHTER_STATUS_KIND_GUARD_ON, // new
        *FIGHTER_STATUS_KIND_GUARD, // new
        *FIGHTER_STATUS_KIND_GUARD_OFF, // new
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE, // new
        *FIGHTER_STATUS_KIND_ATTACK, // new
        *FIGHTER_STATUS_KIND_ATTACK_100, // new
        *FIGHTER_STATUS_KIND_ATTACK_DASH, // new
        *FIGHTER_STATUS_KIND_ATTACK_S3, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI3, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW3, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4, // new
        *FIGHTER_STATUS_KIND_ATTACK_AIR, // new
        *FIGHTER_STATUS_KIND_CATCH, // new
        *FIGHTER_STATUS_KIND_CATCH_PULL, // new
        *FIGHTER_STATUS_KIND_CATCH_DASH, // new
        *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, // new
        *FIGHTER_STATUS_KIND_CATCH_TURN, // new
        *FIGHTER_STATUS_KIND_CATCH_WAIT, // new
        *FIGHTER_STATUS_KIND_CATCH_ATTACK, // new
        *FIGHTER_STATUS_KIND_CATCH_CUT, // new
        *FIGHTER_STATUS_KIND_CATCH_JUMP, // new
        *FIGHTER_STATUS_KIND_THROW, // new
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_OTTOTTO_WAIT,
        *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP,
        *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_ITEM_SCREW_FALL,
        *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP,
        *FIGHTER_STATUS_KIND_ITEM_ROCKETBELT_HOVER_KEEP,
        *FIGHTER_STATUS_KIND_KILLER_JUMP,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_AIR
    ].contains(&status))
}

#[skyline::hook(offset = 0x116db90)]
pub unsafe extern "C" fn shulk_on_attack(vtable: u64, fighter: *mut Fighter, log: &mut CollisionLogScuffed) -> u64 {
    let module_accessor = (*fighter).battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    if WorkModule::is_flag(module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE)
    && WorkModule::get_int(module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) == *FIGHTER_SHULK_MONAD_TYPE_JUMP {
        if [
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP,
            *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL,
            *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_LANDING
        ].contains(&status)
        && log.collision_kind == 1 {
            let opponent_object = MiscModule::get_battle_object_from_id(log.opponent_object_id);
            if !opponent_object.is_null()
            && sv_battle_object::category(log.opponent_object_id) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                VarModule::on_flag(opponent_object, fighter::instance::flag::PURGED);
                VarModule::set_int(opponent_object, fighter::instance::int::PURGED_TIMER, 300);
            }
        }
    }
    original!()(vtable, fighter, log)
}

#[repr(C)]
pub struct CollisionLogScuffed {
    next_log: *const u64,
    end_log: *const u64,
    location: smash_rs::cpp::simd::Vector3,
    x20: u32,
    opponent_object_id: u32,
    x28: u8,
    x29: u8,
    x2a: u8,
    x2b: u8,
    x2c: u8,
    x2d: u8,
    x2e: u8,
    collision_kind: u8,
    receiver_part_id: u8,
    collider_part_id: u8,
    receiver_id: u8,
    collider_id: u8,
    x34: u8,
    x35: bool,
    x36: u8,
    x37: u8,
    x38: u8,
    x39: u8,
    x3a: u8,
    x3b: u8,
    x3c: u8,
    x3d: u8,
    x3e: u8,
    x3f: u8
}

#[skyline::hook(offset = 0x1171c70)]
pub unsafe extern "C" fn shulk_shield_art_hit_decrease(_vtable: u64, _fighter: *mut Fighter, _something: u64) {
    // nothing lol
}

pub fn install() {
    // Disables the Monado Arts wheel by redirecting the instructions to skip the arts wheel check.
    skyline::patching::Patch::in_text(0x1166164).data(0x1400002Au32);

    // Disables a check that causes pushing Special to not bring up the Arts switcher.
    skyline::patching::Patch::in_text(0x1165844).data(0x37000140u32);

    // Disables a weird check that forces you to go into wait/fall while holding Special, probably for the Art Wheel
    skyline::patching::Patch::in_text(0x1167150).data(0x14000499u32);

    // Disables a check that delays the art being selected by holding Special
    skyline::patching::Patch::in_text(0x1165db4).data(0x14000004u32);

    skyline::install_hooks!(
        shulk_check_valid_arts_statuses,
        shulk_on_attack,
        shulk_shield_art_hit_decrease
    );
}