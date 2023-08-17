use super::{currency::Currency, player::Player, price::Price};

fn create_player() -> Player {
    let mut player = Player {
        slug: "slug".to_string(),
        display_name: "name".to_string(),
        prices: vec![],
        birth_date: "2003-07-22T17:15:13Z".to_string(),
        team: Some("team".to_string()),
        stats: None,
        injury: None,
    };

    player.prices.push(Price {
        eur: "40".to_string(),
        usd: "50".to_string(),
        player_slug: "slug".to_string(),
        date: "2023-07-22T17:15:13Z".to_string(),
    });
    player.prices.push(Price {
        eur: "60".to_string(),
        usd: "70".to_string(),
        player_slug: "slug".to_string(),
        date: "2023-07-22T17:15:13Z".to_string(),
    });

    return player;
}

#[test]
fn price_avg_exact_max_count() {
    let player = create_player();

    // Testing for Euro
    match player.price_avg(Currency::Euro, 2) {
        Some(avg) => assert_eq!(avg, 50.0), // Checking if average is corrects
        None => panic!("Unexpected None"),  // In case function returns None
    }

    // Testing for USD
    match player.price_avg(Currency::Usd, 2) {
        Some(avg) => assert_eq!(avg, 60.0), // Checking if average is correct
        None => panic!("Unexpected None"),  // In case function returns None
    }
}

#[test]
fn price_avg_above_max_count() {
    let player = create_player();

    // Testing for Euro
    match player.price_avg(Currency::Euro, 1) {
        Some(avg) => assert_eq!(avg, 40.0), // Checking if average is corrects
        None => panic!("Unexpected None"),  // In case function returns None
    }

    // Testing for USD
    match player.price_avg(Currency::Usd, 1) {
        Some(avg) => assert_eq!(avg, 50.0), // Checking if average is correct
        None => panic!("Unexpected None"),  // In case function returns None
    }
}

#[test]
fn price_avg_under_max_count() {
    let player = create_player();

    // Testing for Euro
    match player.price_avg(Currency::Euro, 5) {
        Some(avg) => assert_eq!(avg, 50.0), // Checking if average is corrects
        None => panic!("Unexpected None"),  // In case function returns None
    }

    // Testing for USD
    match player.price_avg(Currency::Usd, 5) {
        Some(avg) => assert_eq!(avg, 60.0), // Checking if average is correct
        None => panic!("Unexpected None"),  // In case function returns None
    }
}

#[test]
fn age() {
    let player = create_player();
    assert_eq!(player.age(), 20);
}
