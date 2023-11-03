use crate::imports::status_imports::*;
use smash_rs::app::{FighterManager, BraveSetMenuCommand, BraveEnableMenuCommand, BraveShowMenu, BraveSetMenuSelectedCommand};

extern "C" {
    #[link_name = "_ZN3app24FighterSpecializer_Brave30get_special_lw_command_sp_costERKNS_26BattleObjectModuleAccessorENS_28FighterBraveSpecialLwCommandEb"]
    fn get_special_lw_command_sp_cost(boma: *mut BattleObjectModuleAccessor, command: i32, pass_false: bool) -> f32;
}

unsafe fn set_command_for_slot(object: &mut BattleObject, slot: usize, id: i32) {
    let hero_mana = WorkModule::get_float(object.module_accessor, 0x53);
    let mana = get_special_lw_command_sp_cost(object.module_accessor, id, false);
    FighterManager::instance().unwrap().send_event(BraveSetMenuCommand::new(
        WorkModule::get_int(object.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, // ENTRY_ID
        (slot + 1) as u32,
        (id + 1) as i32,
        mana
    ));
    FighterManager::instance().unwrap().send_event(BraveEnableMenuCommand::new(
        WorkModule::get_int(object.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
        (slot + 1) as u32,
        hero_mana >= mana
    ));

    if hero_mana >= mana {
        WorkModule::set_int(object.module_accessor, id, 0x10000d4 + slot as i32);
    }
    else {
        WorkModule::set_int(object.module_accessor, -1, 0x10000d4 + slot as i32);
    }

    VarModule::set_int(object.module_accessor, brave::instance::int::SPELL_SLOT_1 + slot as i32, id);
    *(*(object as *mut _ as *const u64).add(0xd8 / 8) as *mut u8).add(slot) = id as u8;
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
pub unsafe fn roll_spell(mask: i32, current_spells_mask: i32) -> i32 {
    let spell_list_full = [
        (0x0, 10),
        (0x1, 20),
        (0x2, 15),
        (0x3, 20),
        (0x4, 10),
        (0x5, 7),
        (0x6, 3),
        (0x7, 5),
        (0x8, 1),
        (0x9, 4),
        (0xA, 15),
        (0xB, 15),
        (0xC, 20),
        (0xD, 5),
        (0xF, 10),
        (0x10, 10),
        (0x11, 10),
        (0x12, 10),
        (0x13, 5),
        (0x14, 5)
    ];
    let mut rand_max = 0;
    let mut spell_list = vec![];
    let mut hocus_chance = 0;
    let ignore_hocus = sv_math::rand(hash40("object"), 100) < 50;
    for spell in spell_list_full.iter() {
        if (mask | current_spells_mask) & (1 << spell.0) == 0 {
            rand_max += spell.1;
            spell_list.push(*spell);
        }
        else if mask & (1 << spell.0) != 0 && !ignore_hocus {
            hocus_chance += spell.1;
        }
    }
    let mut roll = sv_math::rand(smash::hash40("object"), rand_max + hocus_chance);
    // println!("Roll: {}, Max Roll: {}", roll, rand_max + hocus_chance - 1);
    for spell in spell_list.iter() {
        // println!("Roll {} vs Spell {:#x} cost {}", roll, spell.0, spell.1);
        if roll < spell.1 {
            return spell.0;
        }
        roll -= spell.1;
    }
    0xE
}

#[skyline::hook(replace = FighterSpecializer_Brave::special_lw_open_command)]
unsafe fn special_lw_open_command_hook(object: &mut BattleObject) {
    // println!("b8: {:#x}", *(object as *mut _ as *const u64).add(0xb8 / 8));
    // println!("c8: {:#x}", *(object as *mut _ as *const u32).add(0xc8 / 4));
    // println!("d0: {:#x}", *(object as *mut _ as *const u64).add(0xd0 / 8));
    // println!("c0: {:#x}", *(object as *mut _ as *const u64).add(0xc0 / 8));
    // println!("d8: {:#x}", *(object as *mut _ as *const u64).add(0xd8 / 8));
    // 0x15 = *FIGHTER_BRAVE_SPECIAL_LW_COMMAND_NUM
    // *(*(object as *mut _ as *const u64).add(0xd8 / 8) as *mut u32) = 0x0E0E0E0E;

    *(object as *mut _ as *mut u64).add(0xc0 / 8) = 4; // controls how many commands are active
    *(object as *mut _ as *mut u64).add(0xd0 / 8) = 4;
    WorkModule::set_int(object.module_accessor, 3, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_WINDOW_STATE);
    WorkModule::set_int(object.module_accessor, 0, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_WINDOW_OVERWRITE_FRAME);
    WorkModule::off_flag(object.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_ENABLE_COMMAND_WINDOW_OVERWRITE);
    let index = VarModule::get_int(object.module_accessor, brave::instance::int::NEXT_ROLL_INDEX);

    let mut rolls = vec![];
    // println!("Getting saved spell list...");
    let mask = VarModule::get_int(object.module_accessor, brave::instance::int::USED_SPELL_MASK);
    let mut current_spells = 0;
    for x in 0..4 {
        let mut spell = VarModule::get_int(object.module_accessor, brave::instance::int::SPELL_SLOT_1 + x);
        if spell == 0xE {
            // println!("Hocus Pocus found! Force reroll...");
            spell = -1;
        }
        if spell != -1 {
            current_spells |= 1 << spell;
        }
        // println!("Saved spell on slot {}: {:#x}", x, spell);
        rolls.push(spell);
    }
    for x in 0..4 {
        let mut spell = rolls[x];
        if spell == -1 {
            spell = roll_spell(mask, current_spells);
            current_spells |= 1 << spell;
        }
        rolls[x] = spell;
        set_command_for_slot(object, x as usize, spell);
    }
    WorkModule::set_int(object.module_accessor, index, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
    smash_rs::app::FighterManager::instance().unwrap().send_event(BraveShowMenu::new(
        WorkModule::get_int(object.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32
    ));
    smash_rs::app::FighterManager::instance().unwrap().send_event(BraveSetMenuSelectedCommand::new(
        WorkModule::get_int(object.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
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
unsafe fn lot_critical_hook(object: &mut BattleObject) {
    if WorkModule::is_flag(object.module_accessor, 0x200000EA) {
        WorkModule::on_flag(object.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT);
        return;
    }
    if [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4
    ].contains(&StatusModule::status_kind(object.module_accessor)) {
        original!()(object)
    }
}

pub fn install() {
    skyline::install_hooks!(
        special_lw_open_command_hook,
        get_special_lw_command_sp_cost_hook,
        lot_critical_hook
    );
    
}