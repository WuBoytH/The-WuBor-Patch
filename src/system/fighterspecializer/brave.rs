use crate::imports::*;
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

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum BraveSpellKind {
    HEAL = 0,
    SIZZ,
    SIZZLE,
    BANG,
    KABOOM,
    WHACK,
    THWACK,
    MAGICBURST,
    KAMIKAZEE,
    KACLANG,
    ACCELERATTLE,
    OOMPH,
    BOUNCE,
    SNOOZE,
    HOCUSPOCUS,
    ZOOM,
    FLAMESLASH,
    KACRACKLESLASH,
    METALSLASH,
    HATCHETMAN,
    PSYCHEUP
}

impl BraveSpellKind {
    fn from_i32(value: i32) -> BraveSpellKind {
        match value {
            0 => BraveSpellKind::HEAL,
            1 => BraveSpellKind::SIZZ,
            2 => BraveSpellKind::SIZZLE,
            3 => BraveSpellKind::BANG,
            4 => BraveSpellKind::KABOOM,
            5 => BraveSpellKind::WHACK,
            6 => BraveSpellKind::THWACK,
            7 => BraveSpellKind::MAGICBURST,
            8 => BraveSpellKind::KAMIKAZEE,
            9 => BraveSpellKind::KACLANG,
            10 => BraveSpellKind::ACCELERATTLE,
            11 => BraveSpellKind::OOMPH,
            12 => BraveSpellKind::BOUNCE,
            13 => BraveSpellKind::SNOOZE,
            14 => BraveSpellKind::HOCUSPOCUS,
            15 => BraveSpellKind::ZOOM,
            16 => BraveSpellKind::FLAMESLASH,
            17 => BraveSpellKind::KACRACKLESLASH,
            18 => BraveSpellKind::METALSLASH,
            19 => BraveSpellKind::HATCHETMAN,
            20 => BraveSpellKind::PSYCHEUP,
            _ => panic!("Unknown BraveSpellKind: {}", value),
        }
    }
}

pub unsafe fn check_linked_spells(spell_kind: i32, current_spells_mask: i32) -> Option<i32> {
    let mut mask = current_spells_mask;
    match BraveSpellKind::from_i32(spell_kind) {
        BraveSpellKind::SIZZ => mask |= 1 << BraveSpellKind::SIZZLE as i32,
        BraveSpellKind::SIZZLE => mask |= 1 << BraveSpellKind::SIZZ as i32,
        BraveSpellKind::BANG => mask |= 1 << BraveSpellKind::KABOOM as i32,
        BraveSpellKind::KABOOM => mask |= 1 << BraveSpellKind::BANG as i32,
        BraveSpellKind::WHACK => mask |= 1 << BraveSpellKind::THWACK as i32,
        BraveSpellKind::THWACK => mask |= 1 << BraveSpellKind::WHACK as i32,
        BraveSpellKind::ACCELERATTLE => mask |= 1 << BraveSpellKind::OOMPH as i32,
        BraveSpellKind::OOMPH => mask |= 1 << BraveSpellKind::ACCELERATTLE as i32,
        BraveSpellKind::FLAMESLASH => mask |= 1 << BraveSpellKind::KACRACKLESLASH as i32,
        BraveSpellKind::KACRACKLESLASH => mask |= 1 << BraveSpellKind::FLAMESLASH as i32,
        _v => return None
    }
    Some(mask)
}

pub unsafe fn roll_spell(mask: i32, current_spells_mask: i32) -> i32 {
    let spell_list_full = [
        (BraveSpellKind::HEAL, 10),
        (BraveSpellKind::SIZZ, 20),
        (BraveSpellKind::SIZZLE, 15),
        (BraveSpellKind::BANG, 20),
        (BraveSpellKind::KABOOM, 10),
        (BraveSpellKind::WHACK, 7),
        (BraveSpellKind::THWACK, 3),
        (BraveSpellKind::MAGICBURST, 5),
        (BraveSpellKind::KAMIKAZEE, 1),
        (BraveSpellKind::KACLANG, 4),
        (BraveSpellKind::ACCELERATTLE, 15),
        (BraveSpellKind::OOMPH, 15),
        (BraveSpellKind::BOUNCE, 20),
        (BraveSpellKind::SNOOZE, 5),
        (BraveSpellKind::ZOOM, 10),
        (BraveSpellKind::FLAMESLASH, 10),
        (BraveSpellKind::KACRACKLESLASH, 10),
        (BraveSpellKind::METALSLASH, 10),
        (BraveSpellKind::HATCHETMAN, 5),
        (BraveSpellKind::PSYCHEUP, 5)
    ];
    let mut rand_max = 0;
    let mut spell_list = vec![];
    let mut hocus_chance = 0;
    let ignore_hocus = sv_math::rand(hash40("object"), 100) < 50;
    for spell in spell_list_full.iter() {
        if (mask | current_spells_mask) & (1 << spell.0 as i32) == 0 {
            rand_max += spell.1;
            spell_list.push(*spell);
        }
        else if mask & (1 << spell.0 as i32) != 0 && !ignore_hocus && check_linked_spells(spell.0 as i32, mask).is_none() {
            hocus_chance += spell.1;
        }
    }
    let mut roll = sv_math::rand(smash::hash40("object"), rand_max + hocus_chance);
    // println!("Roll: {}, Max Roll: {}", roll, rand_max + hocus_chance - 1);
    for spell in spell_list.iter() {
        // println!("Roll {} vs Spell {:#x} cost {}", roll, spell.0, spell.1);
        if roll < spell.1 {
            return spell.0 as i32;
        }
        roll -= spell.1;
    }
    BraveSpellKind::HOCUSPOCUS as i32
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
        let mask = VarModule::get_int(object.module_accessor, brave::instance::int::USED_SPELL_MASK);
        let mut spell = rolls[x];
        if spell == -1 {
            spell = roll_spell(mask, current_spells);
            current_spells |= 1 << spell;
            if let Some(new_mask) = check_linked_spells(spell, mask) {
                VarModule::set_int(object.module_accessor, brave::instance::int::USED_SPELL_MASK, new_mask);
            }
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
    if command == BraveSpellKind::KAMIKAZEE as i32 {
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