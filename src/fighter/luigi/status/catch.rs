use crate::imports::status_imports::*;

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_CATCH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn luigi_catch_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Catch()
}

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_CATCH_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn luigi_catch_dash_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchDash()
}

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_CATCH_TURN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn luigi_catch_turn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchTurn()
}

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn luigi_catch_pull_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchPull()
}

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn luigi_catch_pull_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchPull()
}

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_CATCH_DASH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn luigi_catch_dash_pull_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchDashPull()
}

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_CATCH_DASH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn luigi_catch_dash_pull_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchDashPull()
}

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn luigi_catch_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchWait()
}

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn luigi_catch_wait_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchWait()
}

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_CATCH_CUT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn luigi_catch_cut_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchCut()
}

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn luigi_catch_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchAttack()
}

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn luigi_throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Throw()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_CATCH, luigi_catch_end);

    agent.status(smashline::End, *FIGHTER_STATUS_KIND_CATCH_DASH, luigi_catch_dash_end);

    agent.status(smashline::End, *FIGHTER_STATUS_KIND_CATCH_TURN, luigi_catch_turn_end);

    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_main);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_end);

    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, luigi_catch_dash_pull_main);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, luigi_catch_dash_pull_end);

    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_CATCH_WAIT, luigi_catch_wait_main);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_CATCH_WAIT, luigi_catch_wait_end);

    agent.status(smashline::End, *FIGHTER_STATUS_KIND_CATCH_CUT, luigi_catch_cut_end);

    agent.status(smashline::End, *FIGHTER_STATUS_KIND_CATCH_ATTACK, luigi_catch_attack_end);

    agent.status(smashline::End, *FIGHTER_STATUS_KIND_THROW, luigi_throw_end);
}