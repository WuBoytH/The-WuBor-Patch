use crate::imports::status_imports::*;
use smash_rs::app::{FighterManager, BraveSetMenuCommand, BraveEnableMenuCommand, BraveShowMenu, BraveSetMenuSelectedCommand};

extern "C" {
    #[link_name = "_ZN3app24FighterSpecializer_Brave30get_special_lw_command_sp_costERKNS_26BattleObjectModuleAccessorENS_28FighterBraveSpecialLwCommandEb"]
    fn get_special_lw_command_sp_cost(boma: *mut BattleObjectModuleAccessor, command: i32, pass_false: bool) -> f32;
}

unsafe fn set_command_for_slot(fighter: &mut BattleObject, slot: usize, id: i32) {
    let hero_mana = WorkModule::get_float(fighter.module_accessor, 0x53);
    let mana = get_special_lw_command_sp_cost(fighter.module_accessor, id, false);
    FighterManager::instance().unwrap().send_event(BraveSetMenuCommand::new(
        WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, // ENTRY_ID
        (slot + 1) as u32,
        (id + 1) as i32,
        mana
    ));
    FighterManager::instance().unwrap().send_event(BraveEnableMenuCommand::new(
        WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
        (slot + 1) as u32,
        hero_mana >= mana
    ));

    if hero_mana >= mana {
        WorkModule::set_int(fighter.module_accessor, id, 0x10000d4 + slot as i32);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, -1, 0x10000d4 + slot as i32);
    }

    VarModule::set_int(fighter, brave::instance::int::SPELL_SLOT_1 + slot as i32, id);
    *(*(fighter as *mut _ as *const u64).add(0xd8 / 8) as *mut u8).add(slot) = id as u8;
}

// [Command ID List]
// 0x0 - Heal
// 0x1 - Sizz
// 0x2 - Sizzle
// 0x3 - Bang
// 0x4 - Kaboom
// 0x5 - Whack
// 0x6 - Thwack
// 0x7 - Magic Burst
// 0x8 - Kamikazee
// 0x9 - Kaclang
// 0xA - Acceleratle
// 0xB - Oomph
// 0xC - Bounce
// 0xD - Snooze
// 0xE - Hocus Pocus
// 0xF - Zoom
// 0x10 - Flame Slash
// 0x11 - Kacrackle Slash
// 0x12 - Metal Slash
// 0x13 - Hatchet Man
// 0x14 - Psyche Up
pub unsafe fn roll_spell() -> i32 {
    let roll = sv_math::rand(smash::hash40("fighter"), 275);

    // vanilla roll chances
    match roll {
        0..=15 => 0x1,
        16..=35 => 0x2,
        36..=51 => 0x3,
        52..=71 => 0x4,
        72..=88 => 0xD,
        89..=106 => 0x10,
        107..=124 => 0x11,
        125..=131 => 0x12,
        132..=149 => 0x13,
        150..=157 => 0x5,
        158..=169 => 0x6,
        170..=174 => 0x7,
        175..=179 => 0x8,
        180..=195 => 0x14,
        196..=211 => 0xB,
        212..=227 => 0xA,
        228..=232 => 0x9,
        233..=248 => 0xC,
        249..=255 => 0x0,
        256..=270 => 0xF,
        _ => 0xE
    }
}

#[skyline::hook(replace = FighterSpecializer_Brave::special_lw_open_command)]
unsafe fn special_lw_open_command_hook(fighter: &mut BattleObject) {
    // println!("b8: {:#x}", *(fighter as *mut _ as *const u64).add(0xb8 / 8));
    // println!("c8: {:#x}", *(fighter as *mut _ as *const u32).add(0xc8 / 4));
    // println!("d0: {:#x}", *(fighter as *mut _ as *const u64).add(0xd0 / 8));
    // println!("c0: {:#x}", *(fighter as *mut _ as *const u64).add(0xc0 / 8));
    // println!("d8: {:#x}", *(fighter as *mut _ as *const u64).add(0xd8 / 8));
    // 0x15 = *FIGHTER_BRAVE_SPECIAL_LW_COMMAND_NUM
    // *(*(fighter as *mut _ as *const u64).add(0xd8 / 8) as *mut u32) = 0x0E0E0E0E;

    *(fighter as *mut _ as *mut u64).add(0xc0 / 8) = 4; // controls how many commands are active
    *(fighter as *mut _ as *mut u64).add(0xd0 / 8) = 4;
    WorkModule::set_int(fighter.module_accessor, 3, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_WINDOW_STATE);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_WINDOW_OVERWRITE_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_ENABLE_COMMAND_WINDOW_OVERWRITE);
    let index = VarModule::get_int(fighter, brave::instance::int::NEXT_ROLL_INDEX);

    let mut rolls = vec![];
    for x in 0..4 {
        let spell = VarModule::get_int(fighter, brave::instance::int::SPELL_SLOT_1 + x);
        rolls.push(spell);
    }
    for x in 0..4 {
        let mut spell = rolls[x];
        if spell == -1 {
            loop {
                spell = roll_spell();
                if spell == 0xE || !rolls.contains(&spell) {
                    break;
                }
            }
            let mask = VarModule::get_int(fighter, brave::instance::int::USED_SPELL_MASK);
            if mask & (1 << spell) != 0 {
                spell = 0xE;
            }
        }
        rolls[x] = spell;
        set_command_for_slot(fighter, x as usize, spell);
    }
    WorkModule::set_int(fighter.module_accessor, index, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
    smash_rs::app::FighterManager::instance().unwrap().send_event(BraveShowMenu::new(
        WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32
    ));
    smash_rs::app::FighterManager::instance().unwrap().send_event(BraveSetMenuSelectedCommand::new(
        WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
        (index as u32) + 1
    ));
}

#[skyline::hook(replace = FighterSpecializer_Brave::get_special_lw_command_sp_cost)]
unsafe fn get_special_lw_command_sp_cost_hook(module_accessor: *mut BattleObjectModuleAccessor, command: i32, something: bool) -> f32 {
    if command == 0x8 {
        let mut mp = WorkModule::get_float(module_accessor, 0x53);
        if mp <= 1.0 {
            mp = 1.0;
        }
        return mp;
    }
    original!()(module_accessor, command, something)
}

#[skyline::hook(replace = FighterSpecializer_Brave::lot_critical)]
unsafe fn lot_critical_hook(fighter: &mut BattleObject) {
    if WorkModule::is_flag(fighter.module_accessor, 0x200000EA) {
        WorkModule::set_flag(fighter.module_accessor, is_psyche_up, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
        return;
    }
    if [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4
    ].contains(&StatusModule::status_kind(fighter.module_accessor)) {
        original!()(fighter)
    }
}

pub fn install() {
    skyline::install_hooks!(
        special_lw_open_command_hook,
        get_special_lw_command_sp_cost_hook,
        lot_critical_hook
    );
    
}