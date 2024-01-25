use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Set2 {
    /// * `0` - Player name.
    /// * `1` - Bet amount.
    side_1: HashMap<String, u64>,
    /// * `0` - Player name.
    /// * `1` - Bet amount.
    side_2: HashMap<String, u64>,
}

impl Set2 {
    pub fn bet_1(&mut self, player: &mut impl Player, amount: u64) -> Result<(), BetError> {
        self.bet_side(player, amount, Set2Side::Side1)
    }
    pub fn bet_2(&mut self, player: &mut impl Player, amount: u64) -> Result<(), BetError> {
        self.bet_side(player, amount, Set2Side::Side2)
    }

    fn bet_side(
        &mut self,
        player: &mut impl Player,
        amount: u64,
        side: Set2Side,
    ) -> Result<(), BetError> {
        // Pick side dependent on side argument. If this runs into a borrow checker issue for holding a mutable reference for longer than temporary lifetime, it may be possible to extract into a local function to be evaluated each time a mutable reference to side is needed.
        let side = match side {
            Set2Side::Side1 => &mut self.side_1,
            Set2Side::Side2 => &mut self.side_2,
        };

        // Return if insufficient balance.
        if player.balance() < amount {
            return Err(BetError::InsufficientBalance);
        }

        // Return if player already exists.
        if side.contains_key(player.name()) {
            return Err(BetError::PlayerExists);
        }

        // Reduce balance.
        *player.balance_mut() = player.balance().saturating_sub(amount);

        // Add bet.
        side.insert(player.name().clone(), amount);
        Ok(())
    }

    pub fn raise_1(&mut self, player: &mut impl Player, amount: u64) -> Result<(), BetError> {
        self.raise_side(player, amount, Set2Side::Side1)
    }
    pub fn raise_2(&mut self, player: &mut impl Player, amount: u64) -> Result<(), BetError> {
        self.raise_side(player, amount, Set2Side::Side2)
    }

    fn raise_side(
        &mut self,
        player: &mut impl Player,
        amount: u64,
        side: Set2Side,
    ) -> Result<(), BetError> {
        // Pick side dependent on side argument. If this runs into a borrow checker issue for holding a mutable reference for longer than temporary lifetime, it may be possible to extract into a local function to be evaluated each time a mutable reference to side is needed.
        let side = match side {
            Set2Side::Side1 => &mut self.side_1,
            Set2Side::Side2 => &mut self.side_2,
        };

        // Return if insufficient balance.
        if player.balance() < amount {
            return Err(BetError::InsufficientBalance);
        }

        if !side.contains_key(player.name()) {
            return Err(BetError::PlayerNotExists);
        }

        // Reduce balance.
        *player.balance_mut() = player.balance().saturating_sub(amount);

        // Add amount to bet.
        *side.get_mut(player.name()).unwrap() =
            side.get(player.name()).unwrap().saturating_add(amount);
        Ok(())
    }

    pub fn payout<'a>(&'a self, winner: Set2Side) -> HashMap<&'a String, u64> {
        let mut payout = HashMap::new();

        let (winning_side, losing_side) = match winner {
            Set2Side::Side1 => (&self.side_1, &self.side_2),
            Set2Side::Side2 => (&self.side_2, &self.side_1),
        };

        let winning_side_total = winning_side.iter().fold(0, |a, x| a + x.1);
        let losing_side_total = losing_side.iter().fold(0, |a, x| a + x.1);

        let winning_ratio = winning_side_total as f64 / losing_side_total as f64;
        let losing_ratio = losing_side_total as f64 / winning_side_total as f64;

        // The winning side contains only one person, and they've bet less than the total of the losing side, refund the losing side for the percent that the winning player did not bet. None of this applies if there are multiple players who bet on the winning side, as then they must compete with each other for percent payout. This is for incentive to match opposing bets as the first better on a side. It's in a way, emulated odds.
        //
        // For example, if the winning player (singular!) bet 50, and the losing side bet a total of 100, the winning player will get a payout of 50 (their own bet) + 50 (0.5 * losing side bet), and the losing side will be refunded for 50. As the winning player bet 50/100, which is 0.5.
        if winning_side.len() == 1 && winning_side_total < losing_side_total {
            let winning_player = winning_side.iter().next().unwrap();

            payout.insert(
                winning_player.0,
                winning_player.1 + (losing_side_total as f64 * winning_ratio).round() as u64,
            );

            for player in losing_side {
                payout.insert(
                    player.0,
                    (*player.1 as f64 * (1.0 - winning_ratio)).round() as u64,
                );
            }
        } else {
            for player in winning_side {
                payout.insert(
                    player.0,
                    player.1 + (*player.1 as f64 * losing_ratio).round() as u64,
                );
            }
        }

        payout
    }
}

impl Default for Set2 {
    fn default() -> Self {
        Self {
            side_1: HashMap::new(),
            side_2: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Set2Side {
    Side1,
    Side2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BetError {
    InsufficientBalance,
    PlayerExists,
    PlayerNotExists,
}

pub trait Player {
    fn name(&self) -> &String;
    fn balance(&self) -> u64;
    fn balance_mut(&mut self) -> &mut u64;
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct BasicPlayer {
        name: String,
        balance: u64,
    }

    impl Player for BasicPlayer {
        fn name(&self) -> &String {
            &self.name
        }

        fn balance(&self) -> u64 {
            self.balance
        }

        fn balance_mut(&mut self) -> &mut u64 {
            &mut self.balance
        }
    }

    #[test]
    fn set2_insufficient_balance() {
        let mut set2 = Set2::default();
        let mut player = BasicPlayer {
            name: "Sunrosa".into(),
            balance: 1,
        };

        assert_eq!(
            set2.bet_1(&mut player, 50),
            Err(BetError::InsufficientBalance)
        );
        assert_eq!(
            player.balance(),
            1,
            "Player balance was reduced despite being insufficient."
        );
    }

    #[test]
    fn set2_sufficient_balance() {
        let mut set2 = Set2::default();
        let mut player = BasicPlayer {
            name: "Sunrosa".into(),
            balance: 50,
        };

        assert_eq!(set2.bet_1(&mut player, 50), Ok(()));
        assert_eq!(
            player.balance(),
            0,
            "Player balance was not reduced despite the bet being successfully added."
        );
    }

    #[test]
    fn set2_one_winner_payout_0() {
        let mut set2 = Set2::default();
        let mut sunrosa = BasicPlayer {
            name: "Sunrosa".into(),
            balance: 100,
        };
        let mut sammy = BasicPlayer {
            name: "Sammy".into(),
            balance: 100,
        };
        let mut yawn = BasicPlayer {
            name: "Yawn".into(),
            balance: 100,
        };

        set2.bet_1(&mut sunrosa, 50).unwrap();
        set2.bet_2(&mut sammy, 50).unwrap();
        set2.bet_2(&mut yawn, 50).unwrap();

        assert_eq!(sunrosa.balance(), 50);
        assert_eq!(sammy.balance(), 50);
        assert_eq!(yawn.balance(), 50);

        let payout = set2.payout(Set2Side::Side1);
        let mut payout_assert: HashMap<String, u64> = HashMap::new();
        payout_assert.insert("Sunrosa".into(), 100);
        payout_assert.insert("Sammy".into(), 25);
        payout_assert.insert("Yawn".into(), 25);

        assert_eq!(
            payout
                .into_iter()
                .map(|(x, y)| (x.clone(), y))
                .collect::<HashMap<String, u64>>(),
            payout_assert
        )
    }

    #[test]
    fn set2_one_winner_payout_1() {
        let mut set2 = Set2::default();
        let mut sunrosa = BasicPlayer {
            name: "Sunrosa".into(),
            balance: 100,
        };
        let mut sammy = BasicPlayer {
            name: "Sammy".into(),
            balance: 100,
        };
        let mut yawn = BasicPlayer {
            name: "Yawn".into(),
            balance: 100,
        };

        set2.bet_1(&mut sunrosa, 25).unwrap();
        set2.bet_2(&mut sammy, 50).unwrap();
        set2.bet_2(&mut yawn, 50).unwrap();

        assert_eq!(sunrosa.balance(), 75);
        assert_eq!(sammy.balance(), 50);
        assert_eq!(yawn.balance(), 50);

        let payout = set2.payout(Set2Side::Side1);
        let mut payout_assert: HashMap<String, u64> = HashMap::new();
        payout_assert.insert("Sunrosa".into(), 50);
        payout_assert.insert("Sammy".into(), 38);
        payout_assert.insert("Yawn".into(), 38);

        assert_eq!(
            payout
                .into_iter()
                .map(|(x, y)| (x.clone(), y))
                .collect::<HashMap<String, u64>>(),
            payout_assert
        )
    }

    #[test]
    fn set2_multiplayer_0() {
        let mut set2 = Set2::default();
        let mut sunrosa = BasicPlayer {
            name: "Sunrosa".into(),
            balance: 100,
        };
        let mut sammy = BasicPlayer {
            name: "Sammy".into(),
            balance: 100,
        };
        let mut yawn = BasicPlayer {
            name: "Yawn".into(),
            balance: 100,
        };
        let mut river = BasicPlayer {
            name: "River".into(),
            balance: 100,
        };

        set2.bet_1(&mut sunrosa, 25).unwrap();
        set2.bet_1(&mut sammy, 50).unwrap();
        set2.bet_2(&mut yawn, 50).unwrap();
        set2.bet_2(&mut river, 100).unwrap();

        assert_eq!(sunrosa.balance(), 75);
        assert_eq!(sammy.balance(), 50);
        assert_eq!(yawn.balance(), 50);
        assert_eq!(river.balance(), 0);

        let payout = set2.payout(Set2Side::Side1);
        let mut payout_assert: HashMap<String, u64> = HashMap::new();
        payout_assert.insert("Sunrosa".into(), 75);
        payout_assert.insert("Sammy".into(), 150);

        assert_eq!(
            payout
                .into_iter()
                .map(|(x, y)| (x.clone(), y))
                .collect::<HashMap<String, u64>>(),
            payout_assert
        )
    }

    #[test]
    fn set2_multiplayer_1() {
        let mut set2 = Set2::default();
        let mut sunrosa = BasicPlayer {
            name: "Sunrosa".into(),
            balance: 100,
        };
        let mut sammy = BasicPlayer {
            name: "Sammy".into(),
            balance: 100,
        };
        let mut yawn = BasicPlayer {
            name: "Yawn".into(),
            balance: 100,
        };
        let mut river = BasicPlayer {
            name: "River".into(),
            balance: 100,
        };

        set2.bet_1(&mut sunrosa, 25).unwrap();
        set2.bet_1(&mut sammy, 50).unwrap();
        set2.bet_2(&mut yawn, 50).unwrap();
        set2.bet_2(&mut river, 100).unwrap();

        assert_eq!(sunrosa.balance(), 75);
        assert_eq!(sammy.balance(), 50);
        assert_eq!(yawn.balance(), 50);
        assert_eq!(river.balance(), 0);

        let payout = set2.payout(Set2Side::Side2);
        let mut payout_assert: HashMap<String, u64> = HashMap::new();
        payout_assert.insert("Yawn".into(), 75);
        payout_assert.insert("River".into(), 150);

        assert_eq!(
            payout
                .into_iter()
                .map(|(x, y)| (x.clone(), y))
                .collect::<HashMap<String, u64>>(),
            payout_assert
        )
    }
}
