use super::*;
use wubor_utils::offsets;

unsafe extern "C" fn wave_init(_vtable: u64, weapon: *mut smash::app::Weapon, something: u64, _something_2: f32) {
    let module_accessor = (*weapon).battle_object.module_accessor;

    let lr = PostureModule::lr(module_accessor);
    let some_float = *(something as *const f32).add(0x88 / 0x4) * lr;
    let situation;
    if *(something as *const u8).add(0xa8) == 0 {
        *(weapon as *mut bool).add(0x90) = true;
        WorkModule::on_flag(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLAG_TYPE_AIR);
        situation = 2;
    }
    else {
        let angle = WorkModule::get_param_float(module_accessor, hash40("param_wave"), 0x1d5e336640);
        if angle.to_radians() <= some_float {
            situation = 2;
        }
        else {
            situation = 0;
        }
        *(weapon as *mut bool).add(0x90) = false;
        WorkModule::off_flag(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLAG_TYPE_AIR);
    }
    StatusModule::set_situation_kind(module_accessor, SituationKind(situation), false);
    let rot = some_float * 180.0 / 3.1415927;
    PostureModule::set_rot(module_accessor, &Vector3f::new(rot, 0.0, 0.0), 0);

    let (g_speed, a_speed, life) = if *(something as *const i32).add(0xb0 / 0x4) == 0 {
        if situation == 0 {
            (hash40("speed_ground_s"), hash40("speed_air_s"), hash40("lifetime_ground_s"))
        }
        else {
            (hash40("speed_ground_s"), hash40("speed_air_s"), hash40("lifetime_air_s"))
        }
    }
    else {
        if situation == 0 {
            (hash40("speed_ground_w"), hash40("speed_air_w"), hash40("lifetime_ground_w"))
        }
        else {
            (hash40("speed_ground_w"), hash40("speed_air_w"), hash40("lifetime_air_w"))
        }
    };
    let g_speed = WorkModule::get_param_float(module_accessor, hash40("param_wave"), g_speed);
    let a_speed = WorkModule::get_param_float(module_accessor, hash40("param_wave"), a_speed);
    let life = WorkModule::get_param_int(module_accessor, hash40("param_wave"), life);

    WorkModule::set_float(module_accessor, g_speed, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLOAT_SPEED_GROUND);
    WorkModule::set_float(module_accessor, a_speed, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLOAT_SPEED_AIR);
    WorkModule::set_int(module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);

    if WorkModule::is_flag(module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED) {
        WorkModule::set_float(module_accessor, 1.0, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLOAT_SWALLOWED_SPEED_MUL);
    }
    else {
        let energy = KineticModule::get_energy(module_accessor, 0);
        let speed = g_speed * *(energy as *const f32).add(0x20 / 0x4);
        *(energy as *mut u64).add(0x78 / 0x8) = 0;
        *((energy as *mut u8).add(0x70) as *mut Vector2f) = Vector2f::new(speed, 0.0);
        *(energy as *mut bool).add(0x30) = true;
    }

    WorkModule::set_float(module_accessor, rot, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLOAT_GROUND_ANGLE);
    WorkModule::set_int(module_accessor, *(something as *const i32).add(0xb0 / 0x4), *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_INT_STRENGTH);
    WorkModule::set_float(module_accessor, *(something as *mut f32).add(0xac / 0x4), *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLOAT_POS_OFFSET_X);

    if !WorkModule::is_flag(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLAG_TYPE_AIR)
    && WorkModule::get_int(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
        *(weapon as *mut bool).add(0x90) = true;
    }
}

#[repr(C)]
pub struct LinkEvent {
    pub vtable: u64,
    pub id: u32,
    pub padding: u32,
    pub link_event_kind: Hash40,
    pub receiver_boma: *mut BattleObjectModuleAccessor,
    pub sender_id: u32,
    pub no: u32,
    pub result: bool,
    pub padding3: [u8; 3]
}

extern "C" {
    #[link_name = "\u{1}_ZN3app8lua_bind42LinkModule__send_event_parents_struct_implEPNS_26BattleObjectModuleAccessorEiRNS_9LinkEventE"]
    pub fn send_event_parents_struct(
        module_accessor: *mut BattleObjectModuleAccessor,
        arg2: i32,
        arg3: *mut LinkEvent,
    ) -> u64;
}

unsafe extern "C" fn wave_on_hit(vtable: u64, weapon: *mut smash::app::Weapon, collision_mask: u32) -> u64 {
    let module_accessor = (*weapon).battle_object.module_accessor;
    if StatusModule::status_kind(module_accessor) == 1 {
        return 0;
    }

    if StatusModule::status_kind(module_accessor) == 0 {
        if (collision_mask & 7 != 0 && collision_mask & 6 != 0)
        && *(weapon as *mut u8).add(0x97) == 0 {
            let event = &mut LinkEvent {
                vtable: skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x50b43c8,
                id: 10,
                padding: 0,
                link_event_kind: Hash40::new_raw(0x155ac08c24),
                receiver_boma: 0 as *mut BattleObjectModuleAccessor,
                sender_id: 0x50000000,
                no: 0xffffffff,
                result: false,
                padding3: [0; 3]
            };
            send_event_parents_struct(
                module_accessor,
                3,
                event
            );
        }
        if collision_mask & 3 != 0
        || (collision_mask >> 2 & 1 != 0 && *(weapon as *mut u8).add(0x97) == 0) {
            if !WorkModule::is_flag(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLAG_TYPE_AIR) {
                if WorkModule::get_int(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
                    let effect = if !VarModule::is_flag(module_accessor, vars::dolly_wave::status::flag::FINAL_HIT) {
                        Hash40::new("effect_hitstrong")
                    }
                    else {
                        Hash40::new("effect_hitstrong_last")
                    };
                    MotionAnimcmdModule::call_script_single(
                        module_accessor,
                        *WEAPON_ANIMCMD_EFFECT,
                        effect,
                        -1
                    );
                    return 0;
                }
                else {
                    StatusModule::change_status_request(module_accessor, 1, false);
                    return 1;
                }
            }
            if !WorkModule::is_flag(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLAG_HIT_AIR) {
                MotionAnimcmdModule::call_script_single(
                    module_accessor,
                    *WEAPON_ANIMCMD_EFFECT,
                    Hash40::new_raw(0xd1bf80014),
                    -1
                );
                WorkModule::on_flag(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLAG_HIT_AIR);
            }
        }
    }

    MiscModule::normal_weapon_hit_handler(vtable, weapon, collision_mask)
}

#[skyline::from_offset(offsets::weapon::common::HIT_HANDLER_2)]
unsafe extern "C" fn weapon_hit_handler2(
    vtable: u64,
    weapon: &mut smash::app::Weapon,
    param_3: u64,
    log: u64
);

unsafe extern "C" fn dolly_wave_on_hit2(
    vtable: u64,
    weapon: &mut smash::app::Weapon,
    param_3: u64,
    log: u64,
    damage: f32
) {
    weapon_hit_handler2(vtable, weapon, param_3, log);
    let module_accessor = weapon.battle_object.module_accessor;
    let owner_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let battle_object = MiscModule::get_battle_object_from_id(owner_id);
    if !battle_object.is_null() {
        if (*battle_object).kind == *FIGHTER_KIND_DOLLY as u32
        && TeamModule::team_owner_id(module_accessor) as u32 == owner_id {
            let owner_module_accessor = (*battle_object).module_accessor;
            let collision_log = log as *mut CollisionLogScuffed;
            let collision_kind = (*collision_log).collision_kind;
            if [1, 2].contains(&collision_kind) {
                let mul = if collision_kind == 2 {
                    0.1
                }
                else {
                    1.0
                };
                // println!("meter to add: {}", damage * mul);
                crate::helper::add_go(owner_module_accessor, damage * mul);
            }
        }
    }
}

pub fn install() {
    replace_weapon_vtable(
        *WEAPON_KIND_DOLLY_WAVE,
        0x7,
        wave_init as *const () as _
    );
    replace_weapon_vtable(
        *WEAPON_KIND_DOLLY_WAVE,
        0x1d,
        wave_on_hit as *const () as _
    );
    replace_weapon_vtable(
        *WEAPON_KIND_DOLLY_WAVE,
        0x16,
        dolly_wave_on_hit2 as *const () as _
    );
}
