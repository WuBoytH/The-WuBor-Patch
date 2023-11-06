use crate::imports::status_imports::*;

#[repr(C)]
#[derive(PartialEq)]
pub enum AirDashTier {
    Average,
    Bad,
    Good,
    Great,
    Teleport
}

pub unsafe extern "C" fn get_airdash_tier(fighter: &mut L2CFighterCommon) -> AirDashTier {
    // don't do this
    let fighter_kind = fighter.global_table[KIND].get_i32();
    if [
        *FIGHTER_KIND_RIDLEY
    ].contains(&fighter_kind) {
        return AirDashTier::Great;
    }
    if [
        *FIGHTER_KIND_DONKEY,
        *FIGHTER_KIND_KIRBY,
        *FIGHTER_KIND_CAPTAIN,
        *FIGHTER_KIND_DAISY,
        *FIGHTER_KIND_SHEIK,
        *FIGHTER_KIND_MARIOD,
        *FIGHTER_KIND_PICHU,
        *FIGHTER_KIND_FALCO,
        *FIGHTER_KIND_MEWTWO,
        *FIGHTER_KIND_CHROM,
        *FIGHTER_KIND_METAKNIGHT,
        *FIGHTER_KIND_PIT,
        *FIGHTER_KIND_LUCARIO,
        *FIGHTER_KIND_TOONLINK,
        *FIGHTER_KIND_MIISWORDSMAN,
        *FIGHTER_KIND_REFLET,
        *FIGHTER_KIND_DUCKHUNT,
        *FIGHTER_KIND_BAYONETTA,
        *FIGHTER_KIND_BUDDY,
        *FIGHTER_KIND_TANTAN
        ].contains(&fighter_kind) {
        return AirDashTier::Good;
    }
    if [
        *FIGHTER_KIND_SAMUS,
        *FIGHTER_KIND_SAMUSD,
        *FIGHTER_KIND_MARTH,
        *FIGHTER_KIND_YOUNGLINK,
        *FIGHTER_KIND_ROY,
        *FIGHTER_KIND_GAMEWATCH,
        *FIGHTER_KIND_WARIO,
        *FIGHTER_KIND_PFUSHIGISOU,
        *FIGHTER_KIND_SNAKE,
        *FIGHTER_KIND_IKE,
        *FIGHTER_KIND_DIDDY,
        *FIGHTER_KIND_LUCAS,
        *FIGHTER_KIND_SONIC,
        *FIGHTER_KIND_PIKMIN,
        *FIGHTER_KIND_DEDEDE,
        *FIGHTER_KIND_ROBOT,
        *FIGHTER_KIND_WOLF,
        *FIGHTER_KIND_ROCKMAN,
        *FIGHTER_KIND_LITTLEMAC,
        *FIGHTER_KIND_PACMAN,
        *FIGHTER_KIND_KEN,
        *FIGHTER_KIND_KAMUI,
        *FIGHTER_KIND_SIMON,
        *FIGHTER_KIND_RICHTER,
        *FIGHTER_KIND_KROOL,
        *FIGHTER_KIND_SHIZUE,
        *FIGHTER_KIND_GAOGAEN,
        *FIGHTER_KIND_PACKUN,
        *FIGHTER_KIND_BRAVE,
        *FIGHTER_KIND_MASTER,
        *FIGHTER_KIND_PICKEL,
        *FIGHTER_KIND_EFLAME,
        *FIGHTER_KIND_DEMON,
        *FIGHTER_KIND_TRAIL
    ].contains(&fighter_kind) {
        return AirDashTier::Bad
    }
    AirDashTier::Average
}

#[repr(C)]
pub struct AirDashParams {
    pub attack_frame: f32,
    pub cancel_frame: f32
}

pub unsafe extern "C" fn get_airdash_params(fighter: &mut L2CFighterCommon) -> AirDashParams {
    let attack_frame: f32;
    let cancel_frame: f32;
    if get_airdash_tier(fighter) == AirDashTier::Teleport {
        attack_frame = 24.0;
        cancel_frame = 34.0;
    }
    else {
        attack_frame = 14.0;
        cancel_frame = 28.0;
    }
    AirDashParams{attack_frame, cancel_frame}
}

pub unsafe extern "C" fn escape_air_slide_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

pub unsafe extern "C" fn escape_air_slide_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if [
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY
    ].contains(&prev_status) {
        // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND);
        // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_AIR);
        sv_kinetic_energy!(
            controller_set_accel_x_mul,
            fighter,
            0.0
        );
        sv_kinetic_energy!(
            controller_set_accel_x_add,
            fighter,
            0.0
        );
    }
    // if [
    //     *FIGHTER_STATUS_KIND_DAMAGE_FALL,
    //     *FIGHTER_STATUS_KIND_TREAD_FALL
    // ].contains(&prev_status) {
    //     WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND);
    // }
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
    FighterWorkModuleImpl::calc_escape_air_slide_param(fighter.module_accessor, 0.0);
    fighter.setup_escape_air_slide();
    0.into()
}


#[skyline::hook(replace = L2CFighterCommon_setup_escape_air_slide_common)]
pub unsafe extern "C" fn setup_escape_air_slide_common(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue) {
    let mut stickx = param_1.get_f32();
    let mut sticky = param_2.get_f32();
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE {
        let length = sv_math::vec2_length(stickx, sticky);
        let escape_air_slide_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_slide_stick"));
        if length < escape_air_slide_stick {
            stickx = 1.0 * PostureModule::lr(fighter.module_accessor);
            sticky = 0.0;
        }
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        let normalize = sv_math::vec2_normalize(stickx, sticky);
        let mut dirx = normalize.x;
        let mut diry = normalize.y;
        WorkModule::set_float(fighter.module_accessor, dirx, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
        WorkModule::set_float(fighter.module_accessor, diry, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
        let speed_x = 
            KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) +
            KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE)
        ;
        let speed_y = 
            KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) +
            KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE)
        ;
        let escape_air_slide_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_speed"));
        let slide_speed_x = escape_air_slide_speed * speed_x;
        let slide_speed_y = escape_air_slide_speed * speed_y;
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_FREE,
            slide_speed_x,
            slide_speed_y,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            -1.0,
            -1.0
        );
        let escape_air_slide_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_stiff_frame")); // new
        let escape_air_slide_u_stiff_frame = escape_air_slide_stiff_frame;
        let escape_air_slide_d_stiff_frame = escape_air_slide_stiff_frame;
        dirx = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
        diry = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
        let arctangent = diry.atan2(dirx.abs());
        let stiff_lerp = if 0.0 > arctangent.to_degrees() {
            fighter.lerp(
                escape_air_slide_stiff_frame.into(),
                escape_air_slide_d_stiff_frame.into(),
                (arctangent.to_degrees() / 90.0).into()
            )
        }
        else {
            fighter.lerp(
                escape_air_slide_stiff_frame.into(),
                escape_air_slide_u_stiff_frame.into(),
                (arctangent.to_degrees() / 90.0).into()
            )
        };
        let escape_air_slide_stiff_start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_stiff_start_frame"));
        WorkModule::set_float(fighter.module_accessor, escape_air_slide_stiff_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
        let escape_air_slide_back_end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_back_end_frame"));
        let add_xlu = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
        WorkModule::set_int(fighter.module_accessor, escape_air_slide_back_end_frame + add_xlu, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_BACK_END_FRAME);
        WorkModule::set_float(fighter.module_accessor, stiff_lerp.get_f32(), *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
        EffectModule::req_on_joint(
            fighter.module_accessor,
            Hash40::new("sys_smash_flash_s"),
            Hash40::new("hip"),
            &Vector3f{x: 0.0, y: 4.0, z: 8.0},
            &ZERO_VECTOR,
            1.1,
            &Vector3f{x: 18.0, y: 12.0, z: 0.0},
            &ZERO_VECTOR,
            false,
            0,
            0,
            0
        );
    }
}

pub unsafe extern "C" fn escape_air_slide_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_escape_air_slide_common(fighter);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("escape_air_slide"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_EscapeAir_Main as *const () as _))
}

unsafe extern "C" fn sub_escape_air_slide_common(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_NO_WATER_INOUT_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_escape_air_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_escape_air_uniq as *const () as _));
}

pub unsafe extern "C" fn escape_air_slide_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_STATUS_KIND_FALL
    || status == *FIGHTER_STATUS_KIND_LANDING {
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide"));
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        let escape_air_slide_landing_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_slide_landing_speed_max"));
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        // let landing_speed_mul_escape_air_slide = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_speed_mul_escape_air_slide"));
        let mut landing_speed = speed_x;
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        if escape_air_slide_landing_speed_max < landing_speed.abs() {
            landing_speed = escape_air_slide_landing_speed_max * landing_speed.signum();
        }
        let wavedash_mul = 0.85;
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            landing_speed * wavedash_mul,
            speed_y
        );
        if status == *FIGHTER_STATUS_KIND_LANDING {
            if !MotionModule::is_end(fighter.module_accessor) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);
            }
            // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
        }
    }
    else {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        let mut air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        if speed_x.abs() > air_speed_x_stable {
            air_speed_x_stable *= speed_x.signum();
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                air_speed_x_stable,
                speed_y
            );
        }
    }
    0.into()
}

pub unsafe extern "C" fn escape_air_slide_calc_param(fighter: &mut L2CFighterCommon) -> L2CValue {
    FighterWorkModuleImpl::calc_escape_air_slide_param(fighter.module_accessor, 0.0);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            setup_escape_air_slide_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}