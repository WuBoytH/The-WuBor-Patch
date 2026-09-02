use {
    smash::{
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*, app::*},
};

static IKE_VTABLE_ADDRESS : usize = 0x4fc2940;

#[skyline::hook(offset = 0xaf9350)]
pub unsafe extern "C" fn ike_on_attack(_vtable: u64, fighter: &mut Fighter, log: u64) {
    let module_accessor = fighter.battle_object.module_accessor;
    let status;
    let param_hash;
    let collision_log : &mut CollisionLogScuffed = std::mem::transmute(log);
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        status = *FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END;
        param_hash = hash40("param_special_n_kirby");
    }
    else {
        status = *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END;
        param_hash = hash40("param_special_n");
    }
    if StatusModule::status_kind(module_accessor) == status
    && VarModule::is_flag(module_accessor, ike::status::flag::SPECIAL_N_ENABLE_CRITICAL) {
        VarModule::off_flag(module_accessor, ike::status::flag::SPECIAL_N_ENABLE_CRITICAL);
        MiscModule::call_critical(module_accessor, log, 0x23, param_hash, 1, 0, 0, 0, 0);
    }

    if StatusModule::status_kind(module_accessor) == *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_4
    && collision_log.collision_kind == *COLLISION_KIND_HIT as u8 {
        let opponent_object_id = collision_log.opponent_object_id;
        let opponent_object = MiscModule::get_battle_object_from_id(opponent_object_id);
        if HitModule::get_status((*opponent_object).module_accessor, collision_log.receiver_id as i32, 0) == 0 {
            let armor_damage = VarModule::get_float(module_accessor, ike::status::float::SPECIAL_HI_ARMOR_DAMAGE);
            if armor_damage != 0.0 {
                DamageModule::heal(module_accessor, -armor_damage * 1.5, 0);
                SoundModule::play_se(
                    module_accessor,
                    Hash40::new("se_common_lifeup"),
                    true,
                    false,
                    false,
                    false,
                    enSEType(0)
                );
                EffectModule::req_follow(
                    module_accessor,
                    Hash40::new("sys_recovery"),
                    Hash40::new("top"),
                    &Vector3f{x: 0.0, y: 0.0, z: 0.0},
                    &Vector3f{x: 0.0, y: 0.0, z: 0.0},
                    1.0,
                    false,
                    0,
                    0,
                    0,
                    0,
                    0,
                    false,
                    false
                );
                VarModule::set_float(module_accessor, ike::status::float::SPECIAL_HI_ARMOR_DAMAGE, 0.0);
            }
        }
    }
}

unsafe extern "C" fn ike_on_damage(_vtable: u64, fighter: &mut Fighter, log: u64) {
    let object = (*fighter).battle_object;
    let module_accessor = object.module_accessor;
    if [
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2
    ].contains(&StatusModule::status_kind(module_accessor)) {
        if VarModule::is_flag(module_accessor, ike::status::flag::SPECIAL_HI_ARMOR) {
            let damage = *((*(log as *const u64).add(0x10 / 0x8)) as *const f32).add(0x4 / 0x4);
            VarModule::add_float(module_accessor, ike::status::float::SPECIAL_HI_ARMOR_DAMAGE, damage);
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        ike_on_attack
    );

    let _ = skyline::patching::Patch::in_text(IKE_VTABLE_ADDRESS + (68 * 0x8)).data(ike_on_damage as *const () as u64);
}
