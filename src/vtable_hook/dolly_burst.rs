use crate::imports::*;
use std::arch::asm;

static mut BURST_BOMA_PTR : u64 = 0;

#[skyline::hook(offset = 0x975878, inline)]
unsafe extern "C" fn burst_set_pos(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = *ctx.registers[0].x.as_ref();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    BURST_BOMA_PTR = boma as u64;
    let mut offset = WorkModule::get_param_float(boma, hash40("param_super_special"), hash40("burst_offset_z"));
    let num = VarModule::get_int(boma, dolly::status::int::SUPER_SPECIAL_TRIPLE_COUNT).max(1);
    offset *= num as f32;
    // println!("offset: {}", offset as f32);
    asm!("fmov s0, w8", in("w8") offset);
}

#[skyline::hook(offset = 0x9758bc, inline)]
unsafe extern "C" fn burst_set_motion(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = BURST_BOMA_PTR as *mut BattleObjectModuleAccessor;
    let num = VarModule::get_int(boma, dolly::status::int::SUPER_SPECIAL_TRIPLE_COUNT);
    let motion = match num {
        1 => hash40("super_special_triple_1"),
        2 => hash40("super_special_triple_2"),
        3 => hash40("super_special_triple_3"),
        _ => hash40("super_special")
    };
    *ctx.registers[8].x.as_mut() = motion;
}

#[skyline::hook(offset = 0x33df6b0)]
unsafe extern "C" fn burst_init(_vtable: u64, weapon: *mut app::Weapon, something: u64) {
    let module_accessor = (*weapon).battle_object.module_accessor;

    // let mut power = *(something as *mut f32).add(0x88 / 0x4);
    // let mut ratio = *(something as *mut f32).add(0x8c / 0x4);

    let motion = *(something as *const u64).add(0x88 / 0x8);
    // println!("motion: {:#x}", motion);
    WorkModule::set_int64(module_accessor, motion as i64, *WEAPON_DOLLY_BURST_INSTANCE_WORK_ID_INT_MOTION_KIND);

    let is_air = *(something as *const bool).add(0xb0);
    // println!("is_air: {}", is_air);
    WorkModule::set_flag(module_accessor, is_air, *WEAPON_DOLLY_BURST_INSTANCE_WORK_ID_FLAG_AIR);

    if [
        hash40("final2"),
        hash40("final3"),
        // hash40("super_special_triple_2"),
        // hash40("super_special_triple_3"),
    ].contains(&motion) {
        let pos = &mut *(something as *mut smash_rs::cpp::simd::Vector2).add(0x98 / 0x8);
        // println!("pos: {}, {}", pos.vec[0], pos.vec[1]);
        GroundModule::set_shape_safe_pos(module_accessor, &Vector2f{x: pos.vec[0], y: pos.vec[1]});
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

#[skyline::hook(offset = 0x33df890)]
unsafe extern "C" fn burst_on_hit(_vtable: u64, weapon: *mut app::Weapon) -> u64 {
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

    return 0
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x975878).nop();
    skyline::install_hooks!(
        burst_set_pos,
        burst_set_motion,
        burst_init,
        burst_on_hit
    );
}