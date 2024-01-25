#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Set2 {
    side_1: Vec<Bet>,
    side_2: Vec<Bet>,
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
        // Pick side dependent on side argument.
        let side = match side {
            Set2Side::Side1 => &mut self.side_1,
            Set2Side::Side2 => &mut self.side_2,
        };

        // Return if insufficient balance.
        if player.balance() < amount {
            return Err(BetError::InsufficientBalance);
        }

        // Reduce balance.
        *player.balance_mut() = player.balance().saturating_sub(amount);

        // Add bet.
        side.push(Bet {
            player_name: player.name().clone(),
            amount,
        });
        Ok(())
    }
}

impl Default for Set2 {
    fn default() -> Self {
        Self {
            side_1: Vec::new(),
            side_2: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Set2Side {
    Side1,
    Side2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BetError {
    InsufficientBalance,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bet {
    pub player_name: String,
    pub amount: u64,
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
}
