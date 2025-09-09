mod test;

use std::collections::HashMap;

use thiserror::Error;
use uuid::Uuid;

use crate::{
    player::{Player, PlayerUuid},
    Currency,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetUuid(pub Uuid);

/// Betting set with two outcomes. A player can bet on both sides at once.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Set2 {
    //// UUID of the set
    uuid: SetUuid,

    /// UUID of the user that created the set
    creator_uuid: PlayerUuid,

    /// Description of what the set is betting on happening or not happening
    bet_basis: String,

    /// Comments users have placed on this set
    ///
    /// - `K`: Player UUID
    /// - `V`: Comment text
    comments: HashMap<PlayerUuid, String>,

    /// - `K`: Player UUID
    /// - `V`: Bet amount
    side_1_bets: HashMap<PlayerUuid, Currency>,

    /// - `K`: Player UUID
    /// - `V`: Bet amount
    side_2_bets: HashMap<PlayerUuid, Currency>,
}

impl Set2 {
    pub fn new(creator_uuid: PlayerUuid, bet_basis: String) -> Self {
        Self {
            uuid: SetUuid(Uuid::new_v4()),
            creator_uuid,
            bet_basis,
            comments: Default::default(),
            side_1_bets: Default::default(),
            side_2_bets: Default::default(),
        }
    }

    /// Bet as `player` on `side` with `amount`. Reduces the player's balance by amount if they have enough.
    ///
    /// # Errors
    /// * [`InsufficientBalance`](BetError::InsufficientBalance) - Insufficient balance to bet.
    /// * [`PlayerExists`](BetError::PlayerExists) - Player cannot initiate a new bet, as they've already bet.
    pub fn bet(
        &mut self,
        player: &mut (impl Player + ?Sized),
        side: Set2Side,
        amount: Currency,
    ) -> Result<(), BetError> {
        // Pick side dependent on side argument. If this runs into a borrow checker issue for holding a mutable reference for longer than temporary lifetime, it may be possible to extract into a local function to be evaluated each time a mutable reference to side is needed.
        let side = match side {
            Set2Side::Side1 => &mut self.side_1_bets,
            Set2Side::Side2 => &mut self.side_2_bets,
        };

        // Return if insufficient balance.
        if player.balance() < amount {
            return Err(BetError::InsufficientBalance);
        }

        // Return if player already exists.
        if side.contains_key(player.uuid()) {
            return Err(BetError::PlayerExists);
        }

        // Reduce balance.
        *player.balance_mut() = player.balance().saturating_sub(amount);

        // Add bet.
        side.insert(*player.uuid(), amount);
        Ok(())
    }

    /// Raise an already-existing bet on `side` as `player` for `amount`. Reduces the player's balance by amount if they have enough.
    ///
    /// # Errors
    /// - [`InsufficientBalance`](BetError::InsufficientBalance) - Insufficient balance to bet.
    /// - [`PlayerNotExists`](BetError::PlayerNotExists) - Player cannot raise their bet, as they've yet to bet at all.
    pub fn raise(
        &mut self,
        player: &mut (impl Player + ?Sized),
        side: Set2Side,
        amount: Currency,
    ) -> Result<(), BetError> {
        // Pick side dependent on side argument. If this runs into a borrow checker issue for holding a mutable reference for longer than temporary lifetime, it may be possible to extract into a local function to be evaluated each time a mutable reference to side is needed.
        let side = match side {
            Set2Side::Side1 => &mut self.side_1_bets,
            Set2Side::Side2 => &mut self.side_2_bets,
        };

        // Return if insufficient balance.
        if player.balance() < amount {
            return Err(BetError::InsufficientBalance);
        }

        // Return if player does not exist yet.
        if !side.contains_key(player.uuid()) {
            return Err(BetError::PlayerNotExists);
        }

        // Reduce balance.
        *player.balance_mut() = player.balance().saturating_sub(amount);

        // Add amount to bet.
        *side.get_mut(player.uuid()).unwrap() =
            side.get(player.uuid()).unwrap().saturating_add(amount);
        Ok(())
    }

    /// If no bet exists yet for `side` for `player`, make a new bet with `amount`. If a bet already exists for `side` for `player`, raise that bet by `amount`.
    ///
    /// # Errors
    /// - [`InsufficientBalance`](BetError::InsufficientBalance) - Insufficient balance to bet.
    pub fn bet_or_raise(
        &mut self,
        player: &mut (impl Player + ?Sized),
        side: Set2Side,
        amount: Currency,
    ) -> Result<(), BetError> {
        match self.bet(player, side, amount) {
            Err(BetError::PlayerExists) => self.raise(player, side, amount)?,
            x => return x,
        }
        Ok(())
    }

    /// Does `side` contain `player_name`?
    pub fn side_has_player(&self, player_uuid: &PlayerUuid, side: Set2Side) -> bool {
        let side = match side {
            Set2Side::Side1 => &self.side_1_bets,
            Set2Side::Side2 => &self.side_2_bets,
        };

        side.contains_key(player_uuid)
    }

    /// Does the set contain `player_name` on either side?
    pub fn contains_player(&self, player_uuid: &PlayerUuid) -> bool {
        self.side_1_bets.contains_key(player_uuid) || self.side_2_bets.contains_key(player_uuid)
    }

    /// Calculate payout.
    ///
    /// # Returns
    /// - `K`: Player UUID.
    /// - `V`: Amount.
    pub fn payout<'a>(&'a self, winner: Set2Side) -> HashMap<&'a PlayerUuid, Currency> {
        let mut payout = HashMap::new();

        let (winning_side_bets, losing_side_bets) = match winner {
            Set2Side::Side1 => (&self.side_1_bets, &self.side_2_bets),
            Set2Side::Side2 => (&self.side_2_bets, &self.side_1_bets),
        };

        let (winning_side_pot, losing_side_pot) = match winner {
            Set2Side::Side1 => (self.side_1_pot(), self.side_2_pot()),
            Set2Side::Side2 => (self.side_2_pot(), self.side_1_pot()),
        };

        let winning_ratio = winning_side_pot as f64 / losing_side_pot as f64;
        let losing_ratio = losing_side_pot as f64 / winning_side_pot as f64;

        if winning_side_bets.len() == 1 && winning_side_pot < losing_side_pot {
            // The winning side contains only one person, and they've bet less than the total of the losing side, refund the losing side for the percent that the winning player did not bet. None of this applies if there are multiple players who bet on the winning side, as then they must compete with each other for percent payout. This is for incentive to match opposing bets as the first better on a side. It's in a way, emulated odds.
            //
            // For example, if the winning player (singular!) bet 50, and the losing side bet a total of 100, the winning player will get a payout of 50 (their own bet) + 50 (0.5 * losing side bet), and the losing side will be refunded for 50. As the winning player bet 50/100, which is 0.5.

            let winning_player = winning_side_bets.iter().next().unwrap();

            payout.insert(
                winning_player.0,
                winning_player.1 + (losing_side_pot as f64 * winning_ratio).round() as Currency,
            );

            for player in losing_side_bets {
                payout.insert(
                    player.0,
                    (*player.1 as f64 * (1.0 - winning_ratio)).round() as Currency,
                );
            }
        } else {
            for player in winning_side_bets {
                payout.insert(
                    player.0,
                    player.1 + (*player.1 as f64 * losing_ratio).round() as Currency,
                );
            }
        }

        payout
    }

    /// The total amount of currency that has been wagered by all players on side_1
    pub fn side_1_pot(&self) -> Currency {
        self.side_1_bets.iter().fold(0, |acc, cur| acc + cur.1)
    }

    /// The total amount of currency that has been wagered by all players on side_2
    pub fn side_2_pot(&self) -> Currency {
        self.side_2_bets.iter().fold(0, |acc, cur| acc + cur.1)
    }

    /// The total amount of currency that has been wagered on both sides by all players
    pub fn pot_size(&self) -> Currency {
        self.side_1_pot() + self.side_2_pot()
    }
}

/// Side of a [`Set2`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Set2Side {
    Side1,
    Side2,
}

/// An error with making a bet or a raise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum BetError {
    /// Player does not have enough balance to make the bet.
    #[error("Player does not have enough balance to make the bet.")]
    InsufficientBalance,

    /// Player already exists in the set, thus a new bet cannot be placed.
    #[error("Player already exists in the set, thus a new bet cannot be placed.")]
    PlayerExists,

    /// Player does not exist in the set, thus a raise cannot be made.
    #[error("Player does not exist in the set, thus a raise cannot be made.")]
    PlayerNotExists,
}
