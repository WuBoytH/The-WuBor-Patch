use crate::imports::*;

#[skyline::hook(offset = 0x853ce0)]
unsafe extern "C" fn handle_psyche_up_hit(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    if !WorkModule::is_flag(module_accessor, 0x200000ea)
    || !WorkModule::is_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT) {
        return;
    }
    let status = StatusModule::status_kind(module_accessor);
    let mot = MotionModule::motion_kind(module_accessor);
    let attack_s3 = status == *FIGHTER_STATUS_KIND_ATTACK_S3 && mot == hash40("attack_s3_s");
    let attack = status == *FIGHTER_STATUS_KIND_ATTACK && (mot == hash40("attack_11") || mot == hash40("attack_12"));
    let is_hit = AttackModule::is_infliction(module_accessor, 6);
    if attack_s3 || attack {
        if !is_hit {
            return;
        }
        // (*fighter as *const bool).add(0xf9) = true;
        *(fighter as *const Fighter as *mut bool).add(0xf9) = true;
        return;
    }
    if !is_hit && !*(fighter as *const Fighter as *const bool).add(0xf8) {
        return;
    }
    remove_psyche_up(fighter);
}

#[skyline::from_offset(0x853e10)]
extern "C" fn remove_psyche_up(fighter: &mut Fighter);

pub fn install() {
    // Removes a Psyche Up check
    skyline::patching::Patch::in_text(0x8542ec).data(0x14000010u32);

    skyline::install_hooks!(
        handle_psyche_up_hit
    );
}