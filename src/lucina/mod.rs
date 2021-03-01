use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::lua_bind::*;
use smash::hash40;
use smash::app::BattleObjectModuleAccessor;
use smash::app::lua_bind::EffectModule;
use smash_script::*;
use smash_script::macros;
use smash::phx::Hash40;
use smash::app::sv_animcmd;

pub unsafe fn entry_id(module_accessor: &mut BattleObjectModuleAccessor) -> usize {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    return entry_id;
}

static mut _TIME_COUNTER: [i32; 8] = [0; 8];
static mut _ONE_MORE_COUNTER: [i32; 8] = [0; 8];

pub unsafe fn special_effect(module_accessor: &mut BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let pos = smash::phx::Vector3f{x: 0.0, y: 13.0, z: 0.0};
    let rot = smash::phx::Vector3f{x: 0.0, y: 90.0, z: 0.0};
    let onemoreeff: u32 = EffectModule::req_follow(module_accessor, smash::phx::Hash40{hash: hash40("sys_counter_flash")}, smash::phx::Hash40{hash: hash40("top")}, &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
    if SHADOW_FRENZY[entry_id] == false {
        EffectModule::set_rgb(module_accessor, onemoreeff, 5.0, 5.0, 0.0);
    }
    else if SHADOW_FRENZY[entry_id] == true {
        EffectModule::set_rgb(module_accessor, onemoreeff, 2.0, 0.0, 5.0);
    }
}

static mut SPECIAL_AIR_S : [bool; 8] = [false; 8];
static mut SPECIAL_LW : [bool; 8] = [false; 8];
static mut SHADOW_FRENZY : [bool; 8] = [false; 8];
static mut AWAKENING : [bool; 8] = [false; 8];
static mut CAN_ONE_MORE : [bool; 8] = [false; 8];
static mut IS_EX : [bool; 8] = [false; 8];
static mut SP_GAUGE : [f32; 8] = [0.0; 8];
static mut SP_GAUGE_MAX : [f32; 8] = [100.0; 8];
static mut METER_GAIN : [f32; 8] = [0.0; 8];
static mut DAMAGE_TAKEN : [f32; 8] = [0.0; 8];
static mut DAMAGE_TAKEN_PREV : [f32; 8] = [0.0; 8];
static mut CURR_STATUS: [u64 ; 8] = [smash::hash40("wait") ; 8];

pub unsafe fn shadow_id(module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6
    || WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        return true;
    }
    else {
        return false;
    }
}

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn lucina_is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let ret = original!()(module_accessor,term);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter_kind == *FIGHTER_KIND_LUCINA {
        if SPECIAL_AIR_S[entry_id] {
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
                return false;
            }
            else {
                return ret;
            }
        }
        if SPECIAL_LW[entry_id] {
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
                return false;
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }
}

#[fighter_frame( agent = FIGHTER_KIND_LUCINA )]
unsafe fn lucina_frame(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let gfxcoords  = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

    if entry_id < 8 {

        //Meter Control
        
        SPECIAL_LW[entry_id] = true;
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH {
            SPECIAL_AIR_S[entry_id] = false;
            SPECIAL_LW[entry_id] = false;
            _TIME_COUNTER[entry_id] = 0;
            if shadow_id(boma) {
                SHADOW_FRENZY[entry_id] = false;
            }
            else {
                SP_GAUGE_MAX[entry_id] = 100.0;
                AttackModule::set_power_up(boma, 1.0);
                DamageModule::set_damage_mul(boma, 1.0);
                SP_GAUGE[entry_id] = 0.0;
                AWAKENING[entry_id] = false;
            }
        }
        if smash::app::sv_information::is_ready_go() == false {
            DamageModule::set_damage_mul(boma, 1.0);
            SPECIAL_AIR_S[entry_id] = false;
            SPECIAL_LW[entry_id] = false;
            SHADOW_FRENZY[entry_id] = false;
            SP_GAUGE_MAX[entry_id] = 100.0;
            DAMAGE_TAKEN[entry_id] = 0.0;
            DAMAGE_TAKEN_PREV[entry_id] = 0.0;
            _TIME_COUNTER[entry_id] = 0;
            CURR_STATUS[entry_id] = smash::hash40("wait");
            SP_GAUGE[entry_id] = 0.0;
            AWAKENING[entry_id] = false;
        }
        DAMAGE_TAKEN[entry_id] = DamageModule::damage(boma, 0);
        if DAMAGE_TAKEN[entry_id] != DAMAGE_TAKEN_PREV[entry_id] && DAMAGE_TAKEN[entry_id] > DAMAGE_TAKEN_PREV[entry_id] && SHADOW_FRENZY[entry_id] == false {
            METER_GAIN[entry_id] = (DAMAGE_TAKEN[entry_id] - DAMAGE_TAKEN_PREV[entry_id]) * (1.0/6.0);
            if SP_GAUGE[entry_id] + METER_GAIN[entry_id] < SP_GAUGE_MAX[entry_id] {
                SP_GAUGE[entry_id] += METER_GAIN[entry_id];
            }
            else {
                SP_GAUGE[entry_id] = SP_GAUGE_MAX[entry_id];
            }
        }
        DAMAGE_TAKEN_PREV[entry_id] = DAMAGE_TAKEN[entry_id];
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) && SHADOW_FRENZY[entry_id] == false && IS_EX[entry_id] == false {
            METER_GAIN[entry_id] = AttackModule::get_power(boma, 0, false, 1.0, false);
            if shadow_id(boma) == false {
                METER_GAIN[entry_id] *= 0.75;
            }
            if SP_GAUGE[entry_id] + METER_GAIN[entry_id] < SP_GAUGE_MAX[entry_id] {
                SP_GAUGE[entry_id] += METER_GAIN[entry_id];
            }
            else {
                SP_GAUGE[entry_id] = SP_GAUGE_MAX[entry_id];
            }
            // println!("Gained Meter: {}", SP_GAUGE[entry_id]);
        }
        if shadow_id(boma) == true {
            DamageModule::set_damage_mul(boma, 0.92);
            AttackModule::set_power_up(boma, 0.8);
            if SHADOW_FRENZY[entry_id] == true {
                SP_GAUGE[entry_id] -= 1.0/16.0;
            }
        }
        else {
            AttackModule::set_power_up(boma, 1.0);
            if DamageModule::damage(boma, 0) > 100.0 {
                if AWAKENING[entry_id] == false
                && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_DAMAGE
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_DAMAGE_AIR
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_THROWN
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_CAPTURE_WAIT
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_DAMAGE_FLY 
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR 
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U 
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_DAMAGE_FALL
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_FINAL
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_SLEEP
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_ESCAPE_B
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_ESCAPE_F
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_SWALLOWED
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_AIR_LASSO
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_CATCHED_REFLET
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_CATCHED_RIDLEY
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_ATTACK_AIR
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_MISS_FOOT
                && WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI) == false
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_DEAD
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_REBIRTH
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_BURY {
                    SP_GAUGE[entry_id] += 50.0;
                    AWAKENING[entry_id] = true;
                    macros::FT_START_CUTIN(fighter);
                }
            }
        }
        if AWAKENING[entry_id] == true {
            DamageModule::set_damage_mul(boma, 0.8);
            SP_GAUGE_MAX[entry_id] = 150.0;
        }
        if SP_GAUGE[entry_id] <= 0.0{
            SP_GAUGE[entry_id] = 0.0;
            SHADOW_FRENZY[entry_id] = false;
        }
        if SP_GAUGE[entry_id] > SP_GAUGE_MAX[entry_id] {
            SP_GAUGE[entry_id] = SP_GAUGE_MAX[entry_id];
        }

        // Special Lw Check

        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD))
        && MotionModule::motion_kind(boma) != smash::hash40("catch_attack") {
            if CURR_STATUS[entry_id] != MotionModule::motion_kind(boma) {
                CAN_ONE_MORE[entry_id] = true;
                _ONE_MORE_COUNTER[entry_id] = 45;
                CURR_STATUS[entry_id] = MotionModule::motion_kind(boma);
            }
        }
        else {
            CURR_STATUS[entry_id] = smash::hash40("wait");
        }
        if _ONE_MORE_COUNTER[entry_id] >= 0 && CAN_ONE_MORE[entry_id] == true {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) == false {
                if SP_GAUGE[entry_id] > 25.0 && SHADOW_FRENZY[entry_id] == false {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                    if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                        _ONE_MORE_COUNTER[entry_id] = -1;
                        CAN_ONE_MORE[entry_id] = false;
                        SP_GAUGE[entry_id] -= 25.0;
                    }
                }
                else if SP_GAUGE[entry_id] > 12.5 && SHADOW_FRENZY[entry_id] == true {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                    if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                        _ONE_MORE_COUNTER[entry_id] = -1;
                        CAN_ONE_MORE[entry_id] = false;
                        SP_GAUGE[entry_id] -= 12.5;
                    }
                }
            }
            else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) && SP_GAUGE[entry_id] == 100.0 && SHADOW_FRENZY[entry_id] == false && shadow_id(boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, true);
                if StatusModule::status_kind(boma) == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT {
                    _ONE_MORE_COUNTER[entry_id] = -1;
                    CAN_ONE_MORE[entry_id] = false;
                    SHADOW_FRENZY[entry_id] = true;
                }
            }
            _ONE_MORE_COUNTER[entry_id] -= 1;
            if _ONE_MORE_COUNTER[entry_id] < 0 {
                CAN_ONE_MORE[entry_id] = false;
            }
        }
        else if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_THROW {
            let throwframe : f32;
            if MotionModule::motion_kind(boma) == smash::hash40("throw_f") {
                _ONE_MORE_COUNTER[entry_id] = 4;
                throwframe = 18.0;
            }
            else if MotionModule::motion_kind(boma) == smash::hash40("throw_b") {
                _ONE_MORE_COUNTER[entry_id] = 4;
                throwframe = 19.0;
            }
            else if MotionModule::motion_kind(boma) == smash::hash40("throw_hi") {
                _ONE_MORE_COUNTER[entry_id] = 4;
                throwframe = 13.0;
            }
            else if MotionModule::motion_kind(boma) == smash::hash40("throw_lw") {
                _ONE_MORE_COUNTER[entry_id] = 4;
                throwframe = 20.0;
            }
            else{
                throwframe = 20.0;
            }
            if MotionModule::frame(boma) > throwframe && CAN_ONE_MORE[entry_id] == false {
                CAN_ONE_MORE[entry_id] = true;
            }
            if _ONE_MORE_COUNTER[entry_id] > 0 && CAN_ONE_MORE[entry_id] == true {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)  && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) == false {
                    if SP_GAUGE[entry_id] > 25.0 && SHADOW_FRENZY[entry_id] == false {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                            _ONE_MORE_COUNTER[entry_id] = -1;
                            CAN_ONE_MORE[entry_id] = false;
                            SP_GAUGE[entry_id] -= 25.0;
                        }
                    }
                    else if SP_GAUGE[entry_id] > 12.5 && SHADOW_FRENZY[entry_id] == true {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                            _ONE_MORE_COUNTER[entry_id] = -1;
                            CAN_ONE_MORE[entry_id] = false;
                            SP_GAUGE[entry_id] -= 12.5;
                        }
                    }
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) && SP_GAUGE[entry_id] == 100.0 && SHADOW_FRENZY[entry_id] == false && shadow_id(boma) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, true);
                    if StatusModule::status_kind(boma) == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT {
                        _ONE_MORE_COUNTER[entry_id] = -1;
                        CAN_ONE_MORE[entry_id] = false;
                        SHADOW_FRENZY[entry_id] = true;
                    }
                }
                _ONE_MORE_COUNTER[entry_id] -= 1;
                if _ONE_MORE_COUNTER[entry_id] < 0 {
                    CAN_ONE_MORE[entry_id] = false;
                }
            }
        }

        // Meter Effects

        if SP_GAUGE[entry_id] >= 25.0 && SHADOW_FRENZY[entry_id] == false {
            if _TIME_COUNTER[entry_id] < 12 {
                _TIME_COUNTER[entry_id] += 1;
            }
            else {
                if SP_GAUGE[entry_id] >= 25.0 && SP_GAUGE[entry_id] < 50.0 {
                    let onemoreeff: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("haver"), &gfxcoords, &gfxcoords, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                    let onemoreeff2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("havel"), &gfxcoords, &gfxcoords, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rgb(boma, onemoreeff, 0.0, 5.0, 5.0);
                    EffectModule::set_rgb(boma, onemoreeff2, 0.0, 5.0, 5.0);
                    _TIME_COUNTER[entry_id] = 0;
                }
                else if SP_GAUGE[entry_id] >= 50.0 && SP_GAUGE[entry_id] < 75.0 {
                    let onemoreeff: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("haver"), &gfxcoords, &gfxcoords, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                    let onemoreeff2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("havel"), &gfxcoords, &gfxcoords, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rgb(boma, onemoreeff, 0.0, 0.0, 5.0);
                    EffectModule::set_rgb(boma, onemoreeff2, 0.0, 0.0, 5.0);
                    _TIME_COUNTER[entry_id] = 0;
                }
                else if SP_GAUGE[entry_id] >= 75.0 && SP_GAUGE[entry_id] < 100.0 {
                    let onemoreeff: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("haver"), &gfxcoords, &gfxcoords, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                    let onemoreeff2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("havel"), &gfxcoords, &gfxcoords, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rgb(boma, onemoreeff, 5.0, 5.0, 0.0);
                    EffectModule::set_rgb(boma, onemoreeff2, 5.0, 5.0, 0.0);
                    _TIME_COUNTER[entry_id] = 0;
                }
                else if SP_GAUGE[entry_id] >= 100.0 && SP_GAUGE[entry_id] < 125.0 {
                    let onemoreeff: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("haver"), &gfxcoords, &gfxcoords, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                    let onemoreeff2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("havel"), &gfxcoords, &gfxcoords, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rgb(boma, onemoreeff, 5.0, 0.0, 0.0);
                    EffectModule::set_rgb(boma, onemoreeff2, 5.0, 0.0, 0.0);
                    _TIME_COUNTER[entry_id] = 0;
                }
                else if SP_GAUGE[entry_id] >= 125.0 && SP_GAUGE[entry_id] < 150.0 {
                    let onemoreeff: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("haver"), &gfxcoords, &gfxcoords, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                    let onemoreeff2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("havel"), &gfxcoords, &gfxcoords, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rgb(boma, onemoreeff, 2.0, 0.0, 5.0);
                    EffectModule::set_rgb(boma, onemoreeff2, 2.0, 0.0, 5.0);
                    _TIME_COUNTER[entry_id] = 0;
                }
                else{
                    let onemoreeff: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("haver"), &gfxcoords, &gfxcoords, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                    let onemoreeff2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("havel"), &gfxcoords, &gfxcoords, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rgb(boma, onemoreeff, 5.0, 5.0, 5.0);
                    EffectModule::set_rgb(boma, onemoreeff2, 5.0, 5.0, 5.0);
                    _TIME_COUNTER[entry_id] = 0;
                }
            }
        }
        if SHADOW_FRENZY[entry_id] == true {
            if _TIME_COUNTER[entry_id] < 12 {
                _TIME_COUNTER[entry_id] += 1;
            }
            else{
                let onemoreeff: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("haver"), &gfxcoords, &gfxcoords, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                let onemoreeff2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("havel"), &gfxcoords, &gfxcoords, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_rgb(boma, onemoreeff, 2.0, 0.0, 5.0);
                EffectModule::set_rgb(boma, onemoreeff2, 2.0, 0.0, 5.0);
                _TIME_COUNTER[entry_id] = 0;
            }
        }

        // Special S Air Check

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_S && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_LANDING {
            SPECIAL_AIR_S[entry_id] = true;
        }
        else if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
            SPECIAL_AIR_S[entry_id] = false;
        }

        // Shadow Frenzy Check

        if MotionModule::motion_kind(boma) == smash::hash40("appeal_hi_l") || MotionModule::motion_kind(boma) == smash::hash40("appeal_hi_r") {
            if SP_GAUGE[entry_id] == 100.0 && shadow_id(boma) {
                SHADOW_FRENZY[entry_id] = true;
            }
        }
        else if SP_GAUGE[entry_id] == 0.0 {
            SHADOW_FRENZY[entry_id] = false;
        }

        // Move Effects

        // if MotionModule::motion_kind(boma) == smash::hash40("attack_dash") {
        //     if MotionModule::frame(boma) > 7.0 && MotionModule::frame(boma) < 18.0 && IS_EX[entry_id] {
        //         let speed_vector = smash::phx::Vector3f { x: 1.0, y: 0.0, z: 0.0 };
        //         KineticModule::add_speed(boma, &speed_vector);
        //     }
        // } 
        // This is buggin' and I can't find out why =(

        if MotionModule::motion_kind(boma) == smash::hash40("special_s1") {
            if MotionModule::frame(boma) > 6.0 && MotionModule::frame(boma) < 18.0 {
                macros::SET_SPEED_EX(fighter, 2.8, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }

        if MotionModule::motion_kind(boma) == smash::hash40("special_s2_lw") || MotionModule::motion_kind(boma) == smash::hash40("special_s2_hi") {
            if MotionModule::frame(boma) > 0.0 {
                AttackModule::clear_all(boma);
            }
        }

        // Jump Cancels

        if MotionModule::motion_kind(boma) == smash::hash40("attack_hi3")
        || MotionModule::motion_kind(boma) == smash::hash40("attack_12")
        || MotionModule::motion_kind(boma) == smash::hash40("attack_air_n")
        || MotionModule::motion_kind(boma) == smash::hash40("attack_air_hi")
        || MotionModule::motion_kind(boma) == smash::hash40("attack_air_lw") {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < 2 {
                    CancelModule::enable_cancel(boma);
                    _ONE_MORE_COUNTER[entry_id] = -1;
                }
            }
        }

        if MotionModule::motion_kind(boma) == smash::hash40("attack_dash") && IS_EX[entry_id] {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < 2 {
                    CancelModule::enable_cancel(boma);
                    _ONE_MORE_COUNTER[entry_id] = -1;
                }
            }
        }

        // Reset EX

        if IS_EX[entry_id] == true
        && (MotionModule::motion_kind(boma) != smash::hash40("attack_dash")
        || MotionModule::motion_kind(boma) != smash::hash40("special_s1")
        || MotionModule::motion_kind(boma) != smash::hash40("special_air_s2_hi")
        || MotionModule::motion_kind(boma) != smash::hash40("special_air_s2_lw")) {
            IS_EX[entry_id] = false;
        }
    }
}

#[script( agent = "lucina", script = "game_attack11", category = ACMD_GAME )]
unsafe fn lucina_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 15, 0, 35, 2.0, 0.0, 9.4, 6.2, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 361, 15, 0, 35, 2.0, 0.0, 9.4, 8.8, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 180, 12, 0, 25, 2.3, 0.0, 9.4, 12.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 361, 12, 0, 25, 2.3, 0.0, 9.4, 12.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    sv_animcmd::frame(lua_state, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

#[script( agent = "lucina", script = "game_attack12", category = ACMD_GAME )]
unsafe fn lucina_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 6.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.5, 50, 100, 0, 50, 4.2, 3.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 3.5, 50, 100, 0, 50, 3.8, -1.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[script( agent = "lucina", script = "game_attackhi3", category = ACMD_GAME )]
unsafe fn lucina_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 6.0, 71, 60, 0, 65, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(1.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 6.0, 71, 60, 0, 65, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(1.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 6.0, 71, 60, 0, 65, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(0.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 6.0, 71, 60, 0, 65, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(-1.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(lua_state, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

// #[script( agent = "lucina", script = "effect_attackhi3", category = ACMD_EFFECT )]
// unsafe fn lucina_utilteffect(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
//     sv_animcmd::frame(lua_state, 3.0);
//     if macros::is_excute(fighter) {
//         macros::AFTER_IMAGE4_ON_arg29(fighter, 0x111623be1du64, 0x118f2aefa7u64, 12, 0x613f917e1u64, 0.0, 2.5, 0.0, 0x613f917e1u64, 0.0, 21.0, 0.0, true, LUA_VOID, 0x613f917e1u64, 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
//         macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
//     }
//     sv_animcmd::frame(lua_state, 12.0);
//     if macros::is_excute(fighter) {
//         AFTER_IMAGE_OFF(1);
//     }
// }

#[script( agent = "lucina", script = "game_attacklw3", category = ACMD_GAME )]
unsafe fn lucina_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 75, 40, 0, 57, 2.7, 0.0, 2.7, 16.700001, Some(0.0), Some(4.3), Some(9.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 6.0, 75, 40, 0, 57, 2.7, 0.0, 0.0, 8.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(boma, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[script( agent = "lucina", script = "game_attackdash", category = ACMD_GAME )]
unsafe fn lucina_dashattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if SP_GAUGE[entry_id(boma)] >= 25.0 && SHADOW_FRENZY[entry_id(boma)] == false {
                SP_GAUGE[entry_id(boma)] -= 25.0;
                special_effect(boma);
                IS_EX[entry_id(boma)] = true;
            }
            else if SP_GAUGE[entry_id(boma)] >= 6.5 && SHADOW_FRENZY[entry_id(boma)] == true {
                SP_GAUGE[entry_id(boma)] -= 6.5;
                special_effect(boma);
                IS_EX[entry_id(boma)] = true;
            }
            else{
                IS_EX[entry_id(boma)] = false;
            }
        }
        else{
            IS_EX[entry_id(boma)] = false;
        }
    }
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 65, 85, 0, 65, 3.6, 5.0, -1.0, 1.5, Some(1.5), Some(-1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 65, 85, 0, 65, 2.5, 0.0, 2.5, -2.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
    }
    sv_animcmd::frame(lua_state, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[script( agent = "lucina", script = "game_attackairn", category = ACMD_GAME )]
unsafe fn lucina_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 7.0);
    macros::FT_MOTION_RATE(fighter, 0.0833333);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.5, 361, 100, 0, 40, 5.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.5, 361, 100, 0, 40, 5.0, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 4.5, 361, 100, 0, 40, 3.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(lua_state, 31.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::wait(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[script( agent = "lucina", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn lucina_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    macros::FT_MOTION_RATE(fighter, 1.6);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 10.0, 361, 80, 0, 40, 3.0, 1.0, 0.0, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 361, 80, 0, 40, 3.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 10.0, 361, 80, 0, 40, 3.0, 1.0, 0.0, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[script( agent = "lucina", script = "game_attackairb", category = ACMD_GAME )]
unsafe fn lucina_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    macros::FT_MOTION_RATE(fighter, 1.5);
    sv_animcmd::frame(lua_state, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::REVERSE_LR(fighter);
    }
    sv_animcmd::frame(lua_state, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 10.0, 361, 80, 0, 40, 3.5, 0.0, 0.0, 3.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 361, 80, 0, 40, 3.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 10.0, 361, 80, 0, 40, 3.5, 0.0, 0.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(lua_state, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[script( agent = "lucina", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn lucina_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 9.0, 55, 40, 0, 40, 3.5, 1.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.0, 55, 40, 0, 40, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("colonells"), 9.0, 55, 40, 0, 40, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 9.0, 55, 40, 0, 40, 3.5, 1.0, 0.0, 6.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[script( agent = "lucina", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn lucina_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.0, 80, 75, 0, 30, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 9.0, 80, 75, 0, 30, 3.5, 1.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 9.0, 80, 75, 0, 30, 3.5, 1.0, 0.0, 6.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 14.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 55.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[script( agent = "lucina", script = "game_attacks4", category = ACMD_GAME )]
unsafe fn lucina_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5, 361, 85, 0, 30, 4.0, 0.0, 8.5, 10.0, Some(0.0), Some(3.5), Some(22.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[script( agent = "lucina", script = "game_attackhi4", category = ACMD_GAME )]
unsafe fn lucina_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 12.0, 89, 90, 0, 42, 5.8, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 12.0, 89, 90, 0, 42, 4.6, 0.0, 0.0, 7.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 12.0, 89, 90, 0, 42, 5.8, 0.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 125, 100, 155, 0, 5.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(-9.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[script( agent = "lucina", scripts = [ "game_specialnend", "game_specialnendhi", "game_specialnendlw", "game_specialairnend", "game_specialairnendhi", "game_specialairnendlw" ], category = ACMD_GAME)]
unsafe fn lucina_nspecialend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        sv_animcmd::frame(lua_state, 8.0);
        if macros::is_excute(fighter) {
            smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 361, 60, 0, 40, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 361, 60, 0, 40, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 60, 0, 40, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 60, 0, 40, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(lua_state, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(boma, 0, false);
            AttackModule::clear(boma, 1, false);
        }
        sv_animcmd::frame(lua_state, 10.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    else {
        sv_animcmd::frame(lua_state, 1.0);
        macros::FT_MOTION_RATE(fighter, 0.125);
        sv_animcmd::frame(lua_state, 9.0);
        macros::FT_MOTION_RATE(fighter, 2.0);
        sv_animcmd::frame(lua_state, 13.0);
        if macros::is_excute(fighter) {
            smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 60, 0, 40, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 9.0, 361, 60, 0, 40, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 9.0, 361, 60, 0, 40, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(lua_state, 16.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        sv_animcmd::frame(lua_state, 17.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

#[script( agent = "lucina", scripts = [ "game_specialnendmax", "game_specialnendmaxhi", "game_specialnendmaxlw", "game_specialairnendmax", "game_specialairnendmaxhi", "game_specialairnendmaxlw" ], category = ACMD_GAME)]
unsafe fn lucina_nspecialendmax(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        sv_animcmd::frame(lua_state, 8.0);
        if macros::is_excute(fighter) {
            smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(lua_state, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(boma, 0, false);
            AttackModule::clear(boma, 1, false);
        }
        sv_animcmd::frame(lua_state, 10.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    else{
        sv_animcmd::frame(lua_state, 1.0);
        macros::FT_MOTION_RATE(fighter, 0.125);
        sv_animcmd::frame(lua_state, 9.0);
        macros::FT_MOTION_RATE(fighter, 2.0);
        sv_animcmd::frame(lua_state, 13.0);
        if macros::is_excute(fighter) {
            smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 17.0, 60, 100, 0, 0, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 17.0, 60, 100, 0, 0, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(lua_state, 16.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        sv_animcmd::frame(lua_state, 17.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

#[script( agent = "lucina", script = "game_specials1", category = ACMD_GAME )]
unsafe fn lucina_sspecial1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let mut dmg : f32;
    let mut kbg : i32;
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 4.0);
    sv_animcmd::frame(lua_state, 2.0);
    if macros::is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if SP_GAUGE[entry_id(boma)] >= 25.0 && SHADOW_FRENZY[entry_id(boma)] == false {
                SP_GAUGE[entry_id(boma)] -= 25.0;
                special_effect(boma);
                IS_EX[entry_id(boma)] = true;
                macros::FT_MOTION_RATE(fighter, 0.333);
            }
            else if SP_GAUGE[entry_id(boma)] >= 6.5 && SHADOW_FRENZY[entry_id(boma)] == true {
                SP_GAUGE[entry_id(boma)] -= 6.5;
                special_effect(boma);
                IS_EX[entry_id(boma)] = true;
                macros::FT_MOTION_RATE(fighter, 0.333);
            }
            else {
                IS_EX[entry_id(boma)] = false;
            }
        }
        else {
            IS_EX[entry_id(boma)] = false;
        }
    }
    sv_animcmd::frame(lua_state, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        if IS_EX[entry_id(boma)] == true {
            dmg = 18.0;
            kbg = 45;
        }
        else {
            dmg = 14.0;
            kbg = 66;
        }
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 2.5, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(20.0), 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 2.5, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 0.7, 0.0, 8.5, 17.0, Some(0.0), Some(8.5), Some(22.0), 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 0.7, 0.0, 8.5, 23.5, Some(0.0), Some(8.5), Some(28.9), 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        if IS_EX[entry_id(boma)] == true {
            dmg = 13.0;
            kbg = 45;
        }
        else {
            dmg = 9.0;
            kbg = 66;
        }
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 2.5, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(20.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 2.5, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 0.7, 0.0, 8.5, 17.0, Some(0.0), Some(8.5), Some(22.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 0.7, 0.0, 8.5, 23.5, Some(0.0), Some(8.5), Some(28.9), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[script( agent = "lucina", script = "game_specialairs1", category = ACMD_GAME )]
unsafe fn lucina_sspecial1air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        macros::SET_SPEED_EX(fighter, 1.362, 2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    sv_animcmd::frame(lua_state, 46.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

#[script( agent = "lucina", script = "game_specialairs2lw", category = ACMD_GAME )]
unsafe fn lucina_sspecial2lwair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    sv_animcmd::frame(lua_state, 2.0);
    macros::FT_MOTION_RATE(fighter, 0.1);
    sv_animcmd::frame(lua_state, 12.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 13.0);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
        if SP_GAUGE[entry_id(boma)] >= 25.0 && SHADOW_FRENZY[entry_id(boma)] == false {
            SP_GAUGE[entry_id(boma)] -= 25.0;
            special_effect(boma);
            IS_EX[entry_id(boma)] = true;
        }
        else if SP_GAUGE[entry_id(boma)] >= 6.5 && SHADOW_FRENZY[entry_id(boma)] == true {
            SP_GAUGE[entry_id(boma)] -= 6.5;
            special_effect(boma);
             IS_EX[entry_id(boma)] = true;
        }
        else {
            IS_EX[entry_id(boma)] = false;
        }
    }
    else {
        IS_EX[entry_id(boma)] = false;
    }
    sv_animcmd::frame(lua_state, 14.0);
    if macros::is_excute(fighter) {
        let dmg : f32;
        let velx : f32;
        let vely : f32;
        if IS_EX[entry_id(boma)] == true {
            dmg = 12.0;
            velx = 1.75;
            vely = -3.5;
        }
        else {
            dmg = 8.0;
            velx = 1.5;
            vely = -3.0;
        }
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, velx, vely, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), dmg, 40, 90, 0, 60, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), dmg, 40, 90, 0, 60, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), dmg, 40, 60, 0, 60, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("kneel"), dmg, 40, 60, 0, 60, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(lua_state, 50.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 61.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[script( agent = "lucina", script = "game_specialairs2hi", category = ACMD_GAME )]
unsafe fn lucina_sspecial2hiair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 1.5, -2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if SP_GAUGE[entry_id(boma)] >= 25.0 && SHADOW_FRENZY[entry_id(boma)] == false {
                SP_GAUGE[entry_id(boma)] -= 25.0;
                special_effect(boma);
                IS_EX[entry_id(boma)] = true;
            }
            else if SP_GAUGE[entry_id(boma)] >= 6.5 && SHADOW_FRENZY[entry_id(boma)] == true {
                SP_GAUGE[entry_id(boma)] -= 6.5;
                special_effect(boma);
                IS_EX[entry_id(boma)] = true;
            }
            else {
                IS_EX[entry_id(boma)] = false;
            }
        }
        else {
            IS_EX[entry_id(boma)] = false;
        }
    }
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        let dmg : f32;
        let velx : f32;
        let vely : f32;
        if IS_EX[entry_id(boma)] == true {
            dmg = 16.0;
            velx = 2.0;
            vely = -1.5;
        }
        else {
            dmg = 12.0;
            velx = 1.5;
            vely = -3.0;
        }
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, velx, vely, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), dmg, 361, 66, 0, 45, 2.5, 0.0, 6.5, 8.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("top"), dmg, 361, 66, 0, 45, 2.5, 0.0, 3.0, 25.0, Some(0.0), Some(2.7), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), dmg, 361, 66, 0, 45, 0.7, 0.0, 5.6, 17.0, Some(0.0), Some(3.5), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), dmg, 361, 66, 0, 45, 0.7, 0.0, 3.0, 23.5, Some(0.0), Some(1.7), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(boma, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::frame(lua_state, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
    }
    sv_animcmd::frame(lua_state, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[script( agent = "lucina", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME)]
unsafe fn lucina_dspecial(fighter: &mut L2CAgentBase) {

}

#[script( agent = "lucina", scripts = [ "game_speciallwhit", "game_specialairlwhit" ], category = ACMD_GAME )]
unsafe fn lucina_dspecialhit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if shadow_id(boma) {
        macros::FT_START_CUTIN(fighter);
        macros::SLOW_OPPONENT(fighter, 20.0, 8.0);
    }
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 3.4, 90, 0, 60, 40, 5.0, 1.0, 0.0, 1.5, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.4, 90, 0, 60, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("colonells"), 3.4, 90, 0, 60, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 3.4, 90, 0, 60, 40, 5.0, 1.0, 0.0, 7.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 1, true, false);
        AttackModule::set_force_reaction(boma, 2, true, false);
        AttackModule::set_force_reaction(boma, 3, true, false);
        if WorkModule::is_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("se_lucina_criticalhit"));
            AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("se_lucina_criticalhit"));
            AttackModule::set_optional_hit_effect(boma, 2, Hash40::new("se_lucina_criticalhit"));
            AttackModule::set_optional_hit_effect(boma, 3, Hash40::new("se_lucina_criticalhit"));
        }
    }
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[script( agent = "lucina", scripts = [ "game_specialhi", "game_specialairhi" ], category = ACMD_GAME )]
unsafe fn lucina_uspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    sv_animcmd::frame(lua_state, 3.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 5.0);
    macros::FT_MOTION_RATE(fighter, 3.5);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 79, 90, 0, 70, 4.0, 0.0, 8.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 79, 90, 0, 70, 4.0, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
    }
    sv_animcmd::frame(lua_state, 6.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 7.0, 79, 90, 0, 20, 5.0, 0.0, 0.0, 4.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 7.0, 79, 90, 0, 20, 5.0, 0.0, 0.0, -1.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 7.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    sv_animcmd::frame(lua_state, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    smash_script::replace_fighter_frames!(
        lucina_frame
    );
    smash_script::replace_scripts!(
        lucina_jab1,
        lucina_jab2,
        lucina_utilt,
        // lucina_utilteffect,
        lucina_dtilt,
        lucina_dashattack,
        lucina_nair,
        lucina_fair,
        lucina_bair,
        lucina_uair,
        lucina_dair,
        lucina_fsmash,
        lucina_usmash,
        lucina_nspecialend,
        lucina_nspecialendmax,
        lucina_sspecial1,
        lucina_sspecial1air,
        lucina_sspecial2lwair,
        lucina_sspecial2hiair,
        lucina_dspecial,
        lucina_dspecialhit,
        lucina_uspecial
    );
    skyline::install_hook!(lucina_is_enable_transition_term_replace);
}