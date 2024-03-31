use crate::imports::*;

unsafe extern "C" fn brave_special_lw_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let spell_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_DECIDE_COMMAND);
    let mask = VarModule::get_int(fighter.module_accessor, brave::instance::int::USED_SPELL_MASK) | (1 << spell_kind);
    // println!("New Mask: {:#b}", mask);
    VarModule::set_int(fighter.module_accessor, brave::instance::int::USED_SPELL_MASK, mask);
    let index = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
    VarModule::set_int(fighter.module_accessor, brave::instance::int::NEXT_ROLL_INDEX, index);
    VarModule::set_int(fighter.module_accessor, brave::instance::int::SPELL_SLOT_1 + index, -1);
    for x in 0..21 {
        if mask & (1 << x) == 0 {
            break;
        }
        if x == 20 {
            // println!("All spells have been used at least once!");
            VarModule::set_int(fighter.module_accessor, brave::instance::int::USED_SPELL_MASK, 0);
            VarModule::set_int(fighter.module_accessor, brave::instance::int::SPELL_SLOT_1, -1);
            VarModule::set_int(fighter.module_accessor, brave::instance::int::SPELL_SLOT_2, -1);
            VarModule::set_int(fighter.module_accessor, brave::instance::int::SPELL_SLOT_3, -1);
            VarModule::set_int(fighter.module_accessor, brave::instance::int::SPELL_SLOT_4, -1);
        }
    }
    let original = smashline::original_status(smashline::Pre, fighter, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START);
    original(fighter)
}

unsafe extern "C" fn brave_special_lw_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let spell_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_INT_ACTIVE_COMMAND);
    if spell_kind == *FIGHTER_BRAVE_SPECIAL_LW_COMMAND08_FULLBURST {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_FULLBURST_INTERRUPT) {
            ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_CRASH, *WEAPON_BRAVE_CRASH_STATUS_KIND_END1);
        }
    }
    if spell_kind == *FIGHTER_BRAVE_SPECIAL_LW_COMMAND09_CRASH {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_GENERATED_ARTICLE) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
            // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUICIDE_SKILL_RESERVED);
            // if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_DEATH_RESERVE) {
            //     WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRASH2_DEATH_RESERVE);
            // }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_SLOW_WHOLE) {
        let slow_frame = SlowModule::whole_frame(fighter.module_accessor);
        if 0 < slow_frame {
            SlowModule::clear_whole(fighter.module_accessor);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_SLOW_WHOLE);
        }
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_DISABLE_SP_AUTO_RECOVER);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), -1);
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START, brave_special_lw_start_pre);
    agent.status(smashline::End, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START, brave_special_lw_start_end);
}