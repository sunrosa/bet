use uuid::Uuid;

use crate::{
    set2::{BetError, Set2, Set2Side},
    Currency,
};

/// The player that makes bets.
pub trait Player {
    /// The player's name.
    fn name(&self) -> &String;
    /// The player's UUID
    fn uuid(&self) -> &Uuid;
    /// The player's account balance.
    fn balance(&self) -> Currency;
    /// Mutable reference to the player's account balance.
    fn balance_mut(&mut self) -> &mut Currency;

    /// [`bet`](Set2::bet) as player.
    fn bet(&mut self, set2: &mut Set2, side: Set2Side, amount: Currency) -> Result<(), BetError> {
        set2.bet(self, side, amount)
    }

    /// [`raise`](Set2::raise) as player.
    fn raise(&mut self, set2: &mut Set2, side: Set2Side, amount: Currency) -> Result<(), BetError> {
        set2.raise(self, side, amount)
    }

    /// [`bet_or_raise`](Set2::bet_or_raise) as player.
    fn bet_or_raise(
        &mut self,
        set2: &mut Set2,
        side: Set2Side,
        amount: Currency,
    ) -> Result<(), BetError> {
        set2.bet_or_raise(self, side, amount)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BasicPlayer {
    pub name: String,
    pub uuid: Uuid,
    pub balance: Currency,
}

impl BasicPlayer {
    pub fn new(name: String, balance: Currency) -> Self {
        Self {
            name,
            uuid: Uuid::new_v4(),
            balance,
        }
    }
}

impl Player for BasicPlayer {
    fn name(&self) -> &String {
        &self.name
    }

    fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    fn balance(&self) -> Currency {
        self.balance
    }

    fn balance_mut(&mut self) -> &mut Currency {
        &mut self.balance
    }
}
