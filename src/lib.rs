#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Set2 {
    side_1: Vec<Bet>,
    side_2: Vec<Bet>,
}

impl Set2 {
    pub fn bet_1(&mut self, player: &mut impl Player, amount: u64) -> Result<(), BetError> {
        self.bet_side(player, amount, 1)
    }
    pub fn bet_2(&mut self, player: &mut impl Player, amount: u64) -> Result<(), BetError> {
        self.bet_side(player, amount, 2)
    }

    fn bet_side(
        &mut self,
        player: &mut impl Player,
        amount: u64,
        side: u8,
    ) -> Result<(), BetError> {
        let side = if side == 1 {
            &mut self.side_1
        } else {
            &mut self.side_2
        };

        if player.balance() < amount {
            return Err(BetError::InsufficientBalance);
        }

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
    }

    #[test]
    fn set2_bet() {
        let mut set2 = Set2::default();
        let mut player = BasicPlayer {
            name: "Sunrosa".into(),
            balance: 50,
        };

        assert_eq!(set2.bet_1(&mut player, 50), Ok(()));
    }
}
