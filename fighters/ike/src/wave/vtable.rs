use super::*;

// Reimplementation of FUN_7100974d40, the on_init callback of Terry's wave ArticleDescriptor (read from the 13.0.2
// database, addresses re-checked against 13.0.5: the function, its helper, the descriptor table and the record
// vtable are identical). Same generate record protocol as the burst (see dolly/src/burst/vtable.rs); the 0x30 byte
// payload carries the ground line angle, the ground-snapped spawn position and the Power Wave strength instead.

/// FUN_7100975220, the ground-conforming spawn helper. `pos_offset_x` arrives in s0 (this is the float argument that
/// does not show up in the caller's register use). If the owner touches the ground it rotates the x offset along the
/// ground normal (atan2 of the touch normal), raycasts the ground under the resulting spot and, when a line is there,
/// writes the line's angle to `angle`, the spot to `pos` and sets `on_ground`; otherwise `angle` and `on_ground` stay
/// at what the caller passed in (0).
#[skyline::from_offset(0x975220)]
unsafe fn wave_ground_spawn(
    pos_offset_x: f32,
    posture_module: *mut u64,
    ground_module: *mut u64,
    angle: *mut f32,
    pos: *mut Vector4f,
    on_ground: *mut u8
);

/// What the wave puts in the record's payload area (record + 0x88, `unk_78` = 0x30 bytes).
#[repr(C)]
struct WavePayload {
    ground_angle: f32,    // 0x88 atan2 of the ground line normal, 0 when not on ground
    _pad_8c: [u8; 0xc],   // 0x8c left untouched by the original
    ground_pos: Vector4f, // 0x98 spawn position from the helper
    on_ground: u8,        // 0xa8
    _pad_a9: [u8; 3],
    pos_offset_x: f32,    // 0xac param_special_n.pos_offset_x
    strength: i32,        // 0xb0 STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH
    _pad_b4: u32,         // 0xb4 counted in the 0x30 length but never written
}

const _: () = assert!(std::mem::size_of::<WavePayload>() == 0x30);
const _: () = assert!(std::mem::offset_of!(GenerateRecord, motion_kind) == 0x88);

/// Degrees to radians, the exact constant the original multiplies by.
const DEG_TO_RAD: f32 = 0.017453292;

unsafe fn vfunc(object: *const u64, byte_offset: usize) -> usize {
    let vtable = *(object as *const *const usize);
    *vtable.add(byte_offset / 8)
}

/// TeamModule (module accessor +0xd8) vtable +0x48 hands back a pointer to the team number; no lua_bind wrapper.
unsafe fn team_no(module_accessor: *mut BattleObjectModuleAccessor) -> u32 {
    let team_module = *(module_accessor as *const *mut u64).add(0xd8 / 8);
    let get_team: extern "C" fn(*mut u64) -> *const u32 = std::mem::transmute(vfunc(team_module, 0x48));
    *get_team(team_module)
}

// FighterEntry +0xf8 is the controller: +0x3f6c is the y of the main stick, +0x3f84 the y of the second stick.
// The sum is normalized against FighterParamAccessor2 +0x50 -> +0xba4 (up) / +0xba8 (down) and clamped to [-1, 1].
unsafe fn stick_y(fighter_entry: *const u8) -> f32 {
    let control = *(fighter_entry.add(0xf8) as *const *const u8);
    let sum = *(control.add(0x3f6c) as *const f32) + *(control.add(0x3f84) as *const f32);
    if sum == 0.0 {
        return 0.0;
    }
    let params = *((singletons::FighterParamAccessor2() as *const u8).add(0x50) as *const *const u8);
    let range = if sum > 0.0 {
        *(params.add(0xba4) as *const i32) as f32
    }
    else {
        -(*(params.add(0xba8) as *const i32) as f32)
    };
    (sum / range).clamp(-1.0, 1.0)
}

unsafe extern "C" fn ike_wave_init(article: *mut u64, module_accessor: *mut BattleObjectModuleAccessor) -> i32 {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    let mut record = GenerateRecord::new();
    record.owner_id = (*module_accessor).battle_object_id;
    record.team_no = team_no(module_accessor);
    record.lr = PostureModule::lr(module_accessor);

    // The entry table hangs off the first field of the FighterManager object (Ghidra's `*instance_`).
    let fighter_manager = *(singletons::FighterManager() as *const *const u8);
    record.unk_7e = ((*(fighter_manager.add(0xe0) as *const u64) & 7) != 0) as u8;
    if entry_id as u32 >= 8 {
        // The original aborts here.
        panic!("dolly_wave_init: entry id {} out of range", entry_id);
    }
    let fighter_entry = *(fighter_manager.add(0x20 + entry_id as usize * 8) as *const *const u8);
    record.stick_y = stick_y(fighter_entry);
    record.power = WorkModule::get_param_float(module_accessor, hash40("power_special_n"), 0);
    record.power_attr = WorkModule::get_param_float(module_accessor, hash40("power_special_attr"), 0);
    record.unk_78 = 0x30;

    // Payload: the helper's ground data plus the offset and strength the wave reads back.
    let strength = if VarModule::is_flag(module_accessor, vars::ike::status::flag::SPECIAL_LW_WHIFF) {
        0
    }
    else {
        1
    };
    // let pos_offset_x = WorkModule::get_param_float(module_accessor, hash40("param_special_n"), hash40("pos_offset_x"));
    let pos_offset_x = 11.0;
    let posture_module = *(module_accessor as *const *mut u64).add(0x38 / 8);
    let ground_module = *(module_accessor as *const *mut u64).add(0x58 / 8);
    let mut ground_angle = 0.0;
    let mut ground_pos = Vector4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    let mut on_ground = 0u8;
    wave_ground_spawn(pos_offset_x, posture_module, ground_module, &mut ground_angle, &mut ground_pos, &mut on_ground);
    let payload = WavePayload {
        ground_angle,
        _pad_8c: [0; 0xc],
        ground_pos,
        on_ground,
        _pad_a9: [0; 3],
        pos_offset_x,
        strength,
        _pad_b4: 0,
    };
    std::ptr::write((&mut record as *mut GenerateRecord as *mut u8).add(0x88) as *mut WavePayload, payload);

    // Spawn position: owner position plus the scaled (pos_offset_x, pos_offset_y) offset. pos_offset_y only applies
    // when the helper found no ground. The offset is rotated by the owner's z rotation, but the original only uses
    // the rotated y component: x always uses the raw lr * pos_offset_x.
    let owner_pos = *PostureModule::pos(module_accessor);
    let lr = PostureModule::lr(module_accessor);
    let scale = PostureModule::scale(module_accessor);
    // The binding types rot()'s return as u64; it is a *const Vector3f.
    let rot_z = (*(PostureModule::rot(module_accessor, 0) as *const Vector3f)).z;
    // let pos_offset_y = if on_ground != 0 {
    //     0.0
    // }
    // else {
    //     WorkModule::get_param_float(module_accessor, hash40("param_special_n"), hash40("pos_offset_y"))
    // };
    let pos_offset_y = 0.0;
    let offset_y = if pos_offset_x == 0.0 && pos_offset_y == 0.0 {
        0.0
    }
    else {
        let angle = lr * rot_z * DEG_TO_RAD;
        if angle != 0.0 {
            let (sin, cos) = angle.sin_cos();
            sin * pos_offset_x + cos * pos_offset_y
        }
        else {
            pos_offset_y
        }
    };
    // GroundModule vtable +0x388, GetZ; no lua_bind wrapper.
    let ground_z: extern "C" fn(*mut u64) -> f32 = std::mem::transmute(vfunc(ground_module, 0x388));
    record.pos = Vector4f {
        x: owner_pos.x + lr * scale * pos_offset_x,
        y: owner_pos.y + scale * offset_y,
        z: ground_z(ground_module),
        w: 0.0
    };
    record.unk_87 = 1;

    // Article vtable: +0x10 generates the weapon from the record and returns an id, +0x30 is the weapon's object id,
    // +0x18 finishes the generate. BattleObject vtable: +0x30 takes the record, +0x38 the id; the original
    // re-fetches the object between the two.
    let generate: extern "C" fn(*mut u64, *mut GenerateRecord) -> i32 = std::mem::transmute(vfunc(article, 0x10));
    let article_object_id: extern "C" fn(*mut u64) -> u32 = std::mem::transmute(vfunc(article, 0x30));
    let finish: extern "C" fn(*mut u64, *mut GenerateRecord) = std::mem::transmute(vfunc(article, 0x18));

    let id = generate(article, &mut record);

    let weapon = MiscModule::get_battle_object_from_id(article_object_id(article)) as *mut u64;
    let on_record: extern "C" fn(*mut u64, *mut GenerateRecord) = std::mem::transmute(vfunc(weapon, 0x30));
    on_record(weapon, &mut record);

    let weapon = MiscModule::get_battle_object_from_id(article_object_id(article)) as *mut u64;
    let on_id: extern "C" fn(*mut u64, i32) = std::mem::transmute(vfunc(weapon, 0x38));
    on_id(weapon, id);

    finish(article, &mut record);

    // The receivers may have put something in these, so destroy them like the original does.
    MiscModule::destruct_sub_object(&mut record.sub_object_b);
    MiscModule::destruct_sub_object(&mut record.sub_object_a);

    0
}

unsafe extern "C" fn wave_init(_vtable: u64, weapon: *mut smash::app::Weapon, something: u64, _something_2: f32) {
    let module_accessor = (*weapon).battle_object.module_accessor;

    let lr = PostureModule::lr(module_accessor);
    let some_float = *(something as *const f32).add(0x88 / 0x4) * lr;
    let situation;
    if *(something as *const u8).add(0xa8) == 0 {
        // *(weapon as *mut bool).add(0x90) = true;
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
        // *(weapon as *mut bool).add(0x90) = false;
        WorkModule::off_flag(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLAG_TYPE_AIR);
    }
    StatusModule::set_situation_kind(module_accessor, SituationKind(situation), false);
    let rot = some_float * 180.0 / 3.1415927;
    PostureModule::set_rot(module_accessor, &Vector3f::new(rot, 0.0, 0.0), 0);

    *(weapon as *mut bool).add(0x90) = *(something as *const i32).add(0xb0 / 0x4) != 0;

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
}

unsafe extern "C" fn wave_on_hit(vtable: u64, weapon: *mut smash::app::Weapon, collision_mask: u32) -> u64 {
    MiscModule::normal_weapon_hit_handler(vtable, weapon, collision_mask)
}

pub fn install() {
    unsafe {
        replace_article_descriptor_func(
            *FIGHTER_KIND_IKE,
            vars::ike::generate::WAVE,
            Some(ike_wave_init),
            None
        );
        replace_weapon_vtable(
            vars::ike::weapon_id::WAVE,
            0x7,
            wave_init as *const () as _
        );
        replace_weapon_vtable(
            vars::ike::weapon_id::WAVE,
            0x1d,
            wave_on_hit as *const () as _
        );
    }
}
