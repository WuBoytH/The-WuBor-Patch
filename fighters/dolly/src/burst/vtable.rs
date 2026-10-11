use super::*;

unsafe fn vfunc(object: *const u64, byte_offset: usize) -> usize {
    let vtable = *(object as *const *const usize);
    *vtable.add(byte_offset / 8)
}

unsafe fn is_usable_object(object: *mut BattleObject) -> bool {
    if object.is_null() || *(object as *const u8).add(0x3a) < 4 {
        return false;
    }
    let is_dummy: extern "C" fn(*mut BattleObject) -> bool = std::mem::transmute(vfunc(object as _, 0x0));
    !is_dummy(object)
}

unsafe fn is_fighter(object: *mut BattleObject) -> bool {
    (*object).battle_object_id >> 0x1c == *BATTLE_OBJECT_CATEGORY_FIGHTER as u32
}

unsafe fn team_no(module_accessor: *mut BattleObjectModuleAccessor) -> u32 {
    let team_module = *(module_accessor as *const *mut u64).add(0xd8 / 8);
    let get_team: extern "C" fn(*mut u64) -> *const u32 = std::mem::transmute(vfunc(team_module, 0x48));
    *get_team(team_module)
}

unsafe fn ground_center_pos(module_accessor: *mut BattleObjectModuleAccessor) -> Vector4f {
    let ground_module = *(module_accessor as *const *mut u64).add(0x58 / 8);
    #[allow(improper_ctypes_definitions)]
    let center_pos: extern "C" fn(*mut u64) -> smash_rs::cpp::simd::Vector3 = std::mem::transmute(vfunc(ground_module, 0x5a0));
    let pos = center_pos(ground_module);
    Vector4f { x: pos.x(), y: pos.y(), z: pos.z(), w: 0.0 }
}

unsafe fn top_joint_position(module_accessor: *mut BattleObjectModuleAccessor, z_offset: f32) -> Vector4f {
    let offset = Vector4f { x: 0.0, y: 0.0, z: z_offset, w: 0.0 };
    let mut pos = Vector4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    ModelModule::joint_global_position_with_offset(
        module_accessor,
        Hash40::new("top"),
        &offset as *const Vector4f as *const Vector3f,
        &mut pos as *mut Vector4f as *mut Vector3f,
        true
    );
    pos
}

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

unsafe fn owner_power(fighter_entry: *const u8) -> f32 {
    let slot = *fighter_entry.add(0x5918) as u64;
    if slot >= *(fighter_entry.add(0x4150) as *const u64) {
        return 1.0;
    }
    let object = *(fighter_entry.add(0x4160 + slot as usize * 8) as *const *mut BattleObject);
    if object.is_null() {
        return 1.0;
    }
    let is_live: extern "C" fn(*mut BattleObject) -> bool = std::mem::transmute(vfunc(object as _, 0x518));
    if !is_live(object) || !is_usable_object(object) || !is_fighter(object) {
        return 1.0;
    }
    let power: extern "C" fn(*mut BattleObject) -> f32 = std::mem::transmute(vfunc(object as _, 0x300));
    power(object)
}

unsafe extern "C" fn dolly_burst_init(article: *mut u64, module_accessor: *mut BattleObjectModuleAccessor) -> i32 {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let status = StatusModule::status_kind(module_accessor);

    let mut record = GenerateRecord::new();
    record.owner_id = (*module_accessor).battle_object_id;
    record.team_no = team_no(module_accessor);
    record.lr = PostureModule::lr(module_accessor);

    let fighter_manager = *(singletons::FighterManager() as *const *const u8);
    record.unk_7e = ((*(fighter_manager.add(0xe0) as *const u64) & 7) != 0) as u8;
    if entry_id as u32 >= 8 {
        panic!("dolly_burst_init: entry id {} out of range", entry_id);
    }
    let fighter_entry = *(fighter_manager.add(0x20 + entry_id as usize * 8) as *const *const u8);
    record.stick_y = stick_y(fighter_entry);
    record.power_attr = WorkModule::get_param_float(module_accessor, hash40("power_special_attr"), 0);
    record.unk_78 = 0x30;

    let mut is_air = StatusModule::situation_kind(module_accessor) != *SITUATION_KIND_GROUND;
    let pos;

    if status != *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL
    || VarModule::is_flag(module_accessor, vars::dolly::status::flag::SUPER_SPECIAL_TRIPLE) {
        let burst_offset_z = WorkModule::get_param_float(module_accessor, hash40("param_final"), hash40("burst_offset_z"));
        let prev_burst_id = WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_WORK_INT_BURST_OBJECT_ID) as u32;
        if prev_burst_id == *BATTLE_OBJECT_ID_INVALID as u32 {
            pos = top_joint_position(module_accessor, burst_offset_z);
            record.motion_kind = hash40("final");
        }
        else {
            let prev_burst = MiscModule::get_battle_object_from_id(prev_burst_id);
            if is_usable_object(prev_burst) {
                let prev_module_accessor = (*prev_burst).module_accessor;
                pos = top_joint_position(prev_module_accessor, burst_offset_z);
                record.center_pos = ground_center_pos(prev_module_accessor);
                is_air = WorkModule::is_flag(prev_module_accessor, *WEAPON_DOLLY_BURST_INSTANCE_WORK_ID_FLAG_AIR);
            }
            else {
                let burst_num = WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_WORK_INT_BURST_NUM) + 1;
                pos = top_joint_position(module_accessor, burst_num as f32 * burst_offset_z);
                record.center_pos = ground_center_pos(module_accessor);
            }
            record.motion_kind = if StatusModule::status_kind(module_accessor) == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL
            && VarModule::is_flag(module_accessor, vars::dolly::status::flag::SUPER_SPECIAL_TRIPLE) {
                let num = VarModule::get_int(module_accessor, vars::dolly::status::int::SUPER_SPECIAL_TRIPLE_COUNT);
                match num {
                    1 => hash40("super_special_triple_1"),
                    2 => hash40("super_special_triple_2"),
                    3 => hash40("super_special_triple_3"),
                    _ => hash40("super_special")
                }
            }
            else if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_WORK_INT_BURST_NUM) == 1 {
                hash40("final2")
            }
            else {
                hash40("final3")
            };
        }
        record.unk_7c = 1;
        WorkModule::inc_int(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_WORK_INT_BURST_NUM);

        let article_object_id: extern "C" fn(*mut u64) -> u32 = std::mem::transmute(vfunc(article, 0x30));
        WorkModule::set_int(module_accessor, article_object_id(article) as i32, *FIGHTER_DOLLY_STATUS_FINAL_WORK_INT_BURST_OBJECT_ID);

        record.power = owner_power(fighter_entry);

        let owner = MiscModule::get_battle_object_from_id(record.owner_id);
        if is_usable_object(owner) && is_fighter(owner) {
            let get_scale: extern "C" fn(*mut BattleObject, *mut f32, *mut f32) = std::mem::transmute(vfunc(owner as _, 0x310));
            get_scale(owner, &mut record.scale[0], &mut record.scale[1]);
        }
    }
    else {
        let burst_offset_z = WorkModule::get_param_float(module_accessor, hash40("param_super_special"), hash40("burst_offset_z"));
        pos = top_joint_position(module_accessor, burst_offset_z);
        record.motion_kind = hash40("super_special");
        record.power = WorkModule::get_param_float(module_accessor, hash40("power_special_n"), 0);
    }

    record.pos = Vector4f { x: pos.x, y: pos.y, z: pos.z, w: 0.0 };
    record.unk_87 = 1;
    record.is_air = is_air;

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

    MiscModule::destruct_sub_object(&mut record.sub_object_b);
    MiscModule::destruct_sub_object(&mut record.sub_object_a);

    0
}

unsafe extern "C" fn burst_init(_vtable: u64, weapon: *mut smash::app::Weapon, something: u64) {
    let module_accessor = (*weapon).battle_object.module_accessor;

    let motion = *(something as *const u64).add(0x88 / 0x8);
    // println!("motion: {:#x}", motion);
    WorkModule::set_int64(module_accessor, motion, *WEAPON_DOLLY_BURST_INSTANCE_WORK_ID_INT_MOTION_KIND);

    let is_air = *(something as *const bool).add(0xb0);
    // println!("is_air: {}", is_air);
    WorkModule::set_flag(module_accessor, is_air, *WEAPON_DOLLY_BURST_INSTANCE_WORK_ID_FLAG_AIR);

    if [
        hash40("final2"),
        hash40("final3"),
        hash40("super_special_triple_2"),
        hash40("super_special_triple_3"),
    ].contains(&motion) {
        let pos = &mut *(something as *mut smash_rs::cpp::simd::Vector2).add(0x98 / 0x8);
        // println!("pos: {}, {}", pos.x(), pos.y());
        GroundModule::set_shape_safe_pos(module_accessor, &Vector2f{x: pos.x(), y: pos.y()});
    }

    let atack_mul = *(something as *const f32).add(0xa8 / 0x4);
    let reaction_mul = *(something as *const f32).add(0xac / 0x4);
    WorkModule::set_float(module_accessor, atack_mul, 0x1); // WEAPON_INSTANCE_WORK_ID_FLOAT_TEAM_OWNER_ATTACK_MUL
    WorkModule::set_float(module_accessor, reaction_mul, 0x2); // WEAPON_INSTANCE_WORK_ID_FLOAT_TEAM_OWNER_ATTACK_REACTION_MUL

    let stage = smash::app::stage::get_stage_id();
    if stage == *StageID::Dolly_Stadium {
        WorkModule::on_flag(module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    }
}

unsafe extern "C" fn burst_on_hit(_vtable: u64, weapon: *mut smash::app::Weapon) -> u64 {
    let module_accessor = (*weapon).battle_object.module_accessor;

    let motion = MotionModule::motion_kind(module_accessor);

    let event = if motion == hash40("final") {
        0x1e7886b711
    }
    else if motion == hash40("final2") {
        0x1f701d1848
    }
    else if motion == hash40("final3") {
        0x1f071a28de
    }
    else if [
        hash40("super_special"),
        hash40("super_special_triple_3"),
    ].contains(&motion) {
        if WorkModule::is_flag(module_accessor, 0x20000006) { // WEAPON_DOLLY_BURST_INSTANCE_WORK_ID_FLAG_HIT_SUPER_SPECIAL
            return 0;
        }

        let hit_slow_mag = WorkModule::get_param_int(module_accessor, hash40("param_burst"), hash40("hit_slow_mag"));
        let hit_slow_frame = WorkModule::get_param_int(module_accessor, hash40("param_burst"), hash40("hit_slow_frame"));
        SlowModule::set(module_accessor, 0, hit_slow_mag, hit_slow_frame, true, 0x50000000);
        WorkModule::on_flag(module_accessor, 0x20000006);

        0x265df8a12e
    }
    else {
        0
    };

    LinkModule::send_event_parents(module_accessor, 3, Hash40::new_raw(event));

    0
}

pub fn install() {
    replace_article_descriptor_func(
        *FIGHTER_KIND_DOLLY,
        *FIGHTER_DOLLY_GENERATE_ARTICLE_BURST,
        Some(dolly_burst_init),
        None
    );
    replace_weapon_vtable(
        *WEAPON_KIND_DOLLY_BURST,
        0x7,
        burst_init as *const () as _
    );
    replace_weapon_vtable(
        *WEAPON_KIND_DOLLY_BURST,
        0x1d,
        burst_on_hit as *const () as _
    );
}
