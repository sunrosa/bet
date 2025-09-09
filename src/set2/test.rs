#[allow(unused_imports)]
use super::*;

#[cfg(test)]
mod test {
    use crate::player::BasicPlayer;

    use super::*;

    #[test]
    fn set2_insufficient_balance() {
        let mut set2 = Set2::default();
        let mut player = BasicPlayer::new("Sunrosa".into(), 1);

        assert_eq!(
            set2.bet(&mut player, Set2Side::Side2, 50),
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

        assert_eq!(set2.bet(&mut player, Set2Side::Side1, 50), Ok(()));
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

        set2.bet(&mut sunrosa, Set2Side::Side1, 50).unwrap();
        set2.bet(&mut sammy, Set2Side::Side2, 50).unwrap();
        set2.bet(&mut yawn, Set2Side::Side2, 50).unwrap();

        assert_eq!(sunrosa.balance(), 50);
        assert_eq!(sammy.balance(), 50);
        assert_eq!(yawn.balance(), 50);

        let payout = set2.payout(Set2Side::Side1);
        let mut payout_assert: HashMap<String, Currency> = HashMap::new();
        payout_assert.insert("Sunrosa".into(), 100);
        payout_assert.insert("Sammy".into(), 25);
        payout_assert.insert("Yawn".into(), 25);

        assert_eq!(
            payout
                .into_iter()
                .map(|(x, y)| (x.clone(), y))
                .collect::<HashMap<String, Currency>>(),
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

        set2.bet(&mut sunrosa, Set2Side::Side1, 25).unwrap();
        set2.bet(&mut sammy, Set2Side::Side2, 50).unwrap();
        set2.bet(&mut yawn, Set2Side::Side2, 50).unwrap();

        assert_eq!(sunrosa.balance(), 75);
        assert_eq!(sammy.balance(), 50);
        assert_eq!(yawn.balance(), 50);

        let payout = set2.payout(Set2Side::Side1);
        let mut payout_assert: HashMap<String, Currency> = HashMap::new();
        payout_assert.insert("Sunrosa".into(), 50);
        payout_assert.insert("Sammy".into(), 38);
        payout_assert.insert("Yawn".into(), 38);

        assert_eq!(
            payout
                .into_iter()
                .map(|(x, y)| (x.clone(), y))
                .collect::<HashMap<String, Currency>>(),
            payout_assert
        )
    }

    #[test]
    fn set2_one_winner_payout_2() {
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

        set2.bet(&mut sunrosa, Set2Side::Side1, 50).unwrap();
        set2.bet(&mut sammy, Set2Side::Side2, 10).unwrap();
        set2.bet(&mut yawn, Set2Side::Side2, 10).unwrap();

        assert_eq!(sunrosa.balance(), 50);
        assert_eq!(sammy.balance(), 90);
        assert_eq!(yawn.balance(), 90);

        let payout = set2.payout(Set2Side::Side1);
        let mut payout_assert: HashMap<String, Currency> = HashMap::new();
        payout_assert.insert("Sunrosa".into(), 70);

        assert_eq!(
            payout
                .into_iter()
                .map(|(x, y)| (x.clone(), y))
                .collect::<HashMap<String, Currency>>(),
            payout_assert
        )
    }

    #[test]
    fn set2_one_winner_payout_3() {
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

        set2.bet(&mut sunrosa, Set2Side::Side1, 50).unwrap();
        set2.bet(&mut sammy, Set2Side::Side2, 75).unwrap();
        set2.bet(&mut yawn, Set2Side::Side2, 25).unwrap();

        let payout = set2.payout(Set2Side::Side1);
        let mut payout_assert: HashMap<String, Currency> = HashMap::new();
        payout_assert.insert("Sunrosa".into(), 100);
        payout_assert.insert("Sammy".into(), 38);
        payout_assert.insert("Yawn".into(), 13);

        assert_eq!(
            payout
                .into_iter()
                .map(|(x, y)| (x.clone(), y))
                .collect::<HashMap<String, Currency>>(),
            payout_assert
        )
    }

    #[test]
    fn set2_two_winners_v_one_0() {
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

        set2.bet(&mut sunrosa, Set2Side::Side1, 50).unwrap();
        set2.bet(&mut sammy, Set2Side::Side2, 10).unwrap();
        set2.bet(&mut yawn, Set2Side::Side2, 10).unwrap();

        let payout = set2.payout(Set2Side::Side2);

        assert!(!payout.contains_key(&String::from("Sunrosa")));
        assert_eq!(payout[&"Sammy".to_owned()], 35);
        assert_eq!(payout[&"Yawn".to_owned()], 35);
    }

    #[test]
    fn set2_two_winners_v_one_1() {
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

        set2.bet(&mut sunrosa, Set2Side::Side1, 50).unwrap();
        set2.bet(&mut sammy, Set2Side::Side2, 20).unwrap();
        set2.bet(&mut yawn, Set2Side::Side2, 10).unwrap();

        let payout = set2.payout(Set2Side::Side2);

        assert!(!payout.contains_key(&String::from("Sunrosa")));
        assert_eq!(payout[&"Sammy".to_owned()], 53);
        assert_eq!(payout[&"Yawn".to_owned()], 27);
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

        set2.bet(&mut sunrosa, Set2Side::Side1, 25).unwrap();
        set2.bet(&mut sammy, Set2Side::Side1, 50).unwrap();
        set2.bet(&mut yawn, Set2Side::Side2, 50).unwrap();
        set2.bet(&mut river, Set2Side::Side2, 100).unwrap();

        assert_eq!(sunrosa.balance(), 75);
        assert_eq!(sammy.balance(), 50);
        assert_eq!(yawn.balance(), 50);
        assert_eq!(river.balance(), 0);

        let payout = set2.payout(Set2Side::Side1);
        let mut payout_assert: HashMap<String, Currency> = HashMap::new();
        payout_assert.insert("Sunrosa".into(), 75);
        payout_assert.insert("Sammy".into(), 150);

        assert_eq!(
            payout
                .into_iter()
                .map(|(x, y)| (x.clone(), y))
                .collect::<HashMap<String, Currency>>(),
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

        set2.bet(&mut sunrosa, Set2Side::Side1, 25).unwrap();
        set2.bet(&mut sammy, Set2Side::Side1, 50).unwrap();
        set2.bet(&mut yawn, Set2Side::Side2, 50).unwrap();
        set2.bet(&mut river, Set2Side::Side2, 100).unwrap();

        assert_eq!(sunrosa.balance(), 75);
        assert_eq!(sammy.balance(), 50);
        assert_eq!(yawn.balance(), 50);
        assert_eq!(river.balance(), 0);

        let payout = set2.payout(Set2Side::Side2);
        let mut payout_assert: HashMap<String, Currency> = HashMap::new();
        payout_assert.insert("Yawn".into(), 75);
        payout_assert.insert("River".into(), 150);

        assert_eq!(
            payout
                .into_iter()
                .map(|(x, y)| (x.clone(), y))
                .collect::<HashMap<String, Currency>>(),
            payout_assert
        )
    }
}
