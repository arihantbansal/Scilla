use crate::{commands::CommandExec, context::ScillaContext, error::ScillaResult};

/// Commands related to staking operations
#[derive(Debug, Clone)]
pub enum StakeCommand {
    Create,
    Delegate,
    Deactivate,
    Withdraw,
    Merge,
    Split,
    Show,
    History,
    GoBack,
}

impl StakeCommand {
    pub fn spinner_msg(&self) -> &'static str {
        match self {
            StakeCommand::Create => "Creating new stake account…",
            StakeCommand::Delegate => "Delegating stake to validator…",
            StakeCommand::Deactivate => "Deactivating stake (cooldown starting)…",
            StakeCommand::Withdraw => "Withdrawing SOL from deactivated stake…",
            StakeCommand::Merge => "Merging stake accounts…",
            StakeCommand::Split => "Splitting stake into multiple accounts…",
            StakeCommand::Show => "Fetching stake account details…",
            StakeCommand::History => "Fetching stake account history…",
            StakeCommand::GoBack => "Going back…",
        }
    }
}

impl StakeCommand {
    pub async fn process_command(&self, _ctx: &ScillaContext) -> ScillaResult<()> {
        match self {
            StakeCommand::Create => todo!(),
            StakeCommand::Delegate => todo!(),
            StakeCommand::Deactivate => todo!(),
            StakeCommand::Withdraw => todo!(),
            StakeCommand::Merge => todo!(),
            StakeCommand::Split => todo!(),
            StakeCommand::Show => todo!(),
            StakeCommand::History => todo!(),
            StakeCommand::GoBack => Ok(CommandExec::GoBack),
        }
    }
}
