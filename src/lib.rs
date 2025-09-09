//! Library backing [`bet-server`](https://github.com/sunrosa/bet-server)
//!
//! # Winning distribution
//! Regarding the winner, winnings are distributed according to multiple factors.
//!
//! In the case of one person on one side winning the bet (the opposite side's number of betters is irrelevant), the winnings will be proportional to the ratio between the winning side's total wager, and the losing side's total wager. For example, if the solo on the winning side wagered 50 credits, while the losing side wagered a total of 100 credits amongst themselves, the winner would win 100 credits (that's including what they originally wagered), and the losing side would be refunded 50, distributed proportionally. The reason for this proportionality for solos is to prevent the strategy of spamming bets of 1 credit across as many bets as possible, just to win big here and there (this game is meant to be played in small groups).
//!
//! In the case of >=2 people on the winning side, winnings are distributed proportional to the players that bet on that side only. In other words, a side that wins, with >=2 people that bet on that side, will always take all from the losing side, regardless of how much each person bet.
//!
//! It's notable that spamming bets of 1 is still a relevant strategy in the case of multiple players betting on the winning side. However, this strategy is countered by placing just a slightly larger wager (e.g. 10) on a bet suspected to be multiplayer (bets are invisible until betting is closed).
//!
//! Regardless, if this spamming remains a problem, minimum bets could be enforced. 3 options would be to scale minimum wagers with the number of bets a person has currently active, or the scale minimum wagers with the person's balance, or both at once, or to hard-cap maximum active bets (that sucks though).
//!
//! # Notes
//! - Betting should be blind. Players betting solo are at a disadvantage, and should always start low until they see their opponents' bets. Instead, if all bets are blind until betting is closed, this problem is avoided.
//! - After betting is over, a timeline of bets could be shown.
//! - N-sided sets are probably not ideal for small playgroups. A lot of the time, there may be no winner. Still, it should be implemented eventually.

pub mod player;
pub mod set2;

pub type Currency = u64;
