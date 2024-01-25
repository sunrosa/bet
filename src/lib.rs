#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Set<P>
where
    P: Player,
{
    pub side_1: Vec<Bet<P>>,
    pub side_2: Vec<Bet<P>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bet<P>
where
    P: Player,
{
    player: P,
    amount: u64,
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
pub struct BasicPlayer {
    pub name: String,
}

impl Player for BasicPlayer {
    fn name(&self) -> &String {
        &self.name
    }
}

pub trait Player {
    fn name(&self) -> &String;
}
