#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Set2<P>
where
    P: Player,
{
    side_1: Vec<Bet<P>>,
    side_2: Vec<Bet<P>>,
}

impl<P> Set2<P>
where
    P: Player,
{
    pub fn bet_1(&mut self, player: P, amount: u64) -> Result<(), BetError> {
        self.bet_side(player, amount, 1)
    }
    pub fn bet_2(&mut self, player: P, amount: u64) -> Result<(), BetError> {
        self.bet_side(player, amount, 2)
    }

    fn bet_side(&mut self, player: P, amount: u64, side: u8) -> Result<(), BetError> {
        let side = if side == 1 {
            &mut self.side_1
        } else {
            &mut self.side_2
        };

        if player.balance() < amount {
            return Err(BetError::InsufficientBalance);
        }

        side.push(Bet::new(player, amount));
        Ok(())
    }
}

pub enum BetError {
    InsufficientBalance,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bet<P>
where
    P: Player,
{
    pub player: P,
    pub amount: u64,
}

impl<P> Bet<P>
where
    P: Player,
{
    pub fn new(player: P, amount: u64) -> Self {
        Self { player, amount }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BasicPlayer {
    pub name: String,
    pub balance: u64,
}

impl Player for BasicPlayer {
    fn name(&self) -> &String {
        &self.name
    }

    fn balance(&self) -> u64 {
        self.balance
    }
}

pub trait Player {
    fn name(&self) -> &String;
    fn balance(&self) -> u64;
}
