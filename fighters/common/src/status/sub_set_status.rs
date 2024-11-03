use super::{
    *,
    escape::escape_air_slide::*,
    cliff::cliff_jump1::*
};

pub fn install() {
    let agent = &mut Agent::new("fighter");

    agent.status(Pre, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, escape_air_slide_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, escape_air_slide_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, escape_air_slide_main);
    agent.status(End, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, escape_air_slide_end);
    agent.status(CalcParam, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, escape_air_slide_calc_param);

    agent.status(Exec, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, sub_cliff_jump1_uniq_process_exec);

    agent.install();
}