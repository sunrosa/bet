#![cfg(test)]

use crate::player::BasicPlayer;

#[allow(unused_imports)]
use super::*;

#[allow(clippy::enum_glob_use)]
use Set2Side::*;

#[test]
fn set2_1v1_payout() {
    let mut ibis = BasicPlayer::new("Ibis".into(), 1000);
    let mut indigo = BasicPlayer::new("Indigo".into(), 1000);

    {
        let mut set = Set2::new(PlayerUuid(Uuid::new_v4()), "BASIS".into());
        set.bet(&mut ibis, Side1, 10).unwrap();
        set.bet(&mut indigo, Side2, 25).unwrap();

        let payout = set.payout(Side2);

        assert!(!payout.contains_key(ibis.uuid()));
        assert_eq!(payout[indigo.uuid()], 35);

        let payout = set.payout(Side1);

        assert_eq!(payout[ibis.uuid()], 20);
        assert_eq!(payout[indigo.uuid()], 15);
    }
    {
        let mut set = Set2::new(PlayerUuid(Uuid::new_v4()), "BASIS".into());
        set.bet(&mut ibis, Side1, 1).unwrap();
        set.bet(&mut indigo, Side2, 25).unwrap();

        let payout = set.payout(Side2);

        assert!(!payout.contains_key(ibis.uuid()));
        assert_eq!(payout[indigo.uuid()], 26);

        let payout = set.payout(Side1);

        assert_eq!(payout[ibis.uuid()], 2);
        assert_eq!(payout[indigo.uuid()], 24);
    }

    assert_eq!(ibis.balance, 989);
    assert_eq!(indigo.balance, 950)
}

#[test]
fn set2_1v2_payout() {
    let mut ibis = BasicPlayer::new("Ibis".into(), 1000);
    let mut indigo = BasicPlayer::new("Indigo".into(), 1000);
    let mut yam = BasicPlayer::new("Yam".into(), 1000);

    {
        let mut set = Set2::new(PlayerUuid(Uuid::new_v4()), "BASIS".into());
        set.bet(&mut ibis, Side1, 10).unwrap();
        set.bet(&mut indigo, Side2, 25).unwrap();
        set.bet(&mut yam, Side2, 25).unwrap();

        let payout = set.payout(Side1);

        assert_eq!(payout[ibis.uuid()], 20);
        assert_eq!(payout[indigo.uuid()], 20);
        assert_eq!(payout[yam.uuid()], 20);

        set.raise(&mut ibis, Side1, 15).unwrap();
        let payout = set.payout(Side1);

        assert_eq!(payout[ibis.uuid()], 50);
        assert_eq!(payout[indigo.uuid()], 13);
        assert_eq!(payout[yam.uuid()], 13);
    }
    {
        let mut set = Set2::new(PlayerUuid(Uuid::new_v4()), "BASIS".into());
        set.bet(&mut ibis, Side1, 10).unwrap();
        set.bet(&mut indigo, Side2, 25).unwrap();
        set.bet(&mut yam, Side2, 40).unwrap();

        let payout = set.payout(Side2);

        assert_eq!(payout[indigo.uuid()], 29);
        assert_eq!(payout[yam.uuid()], 46);
    }
}
