pub enum TurnChoice {
    Split,
    Hit(u8,u8)
}

pub fn display_game_state((p1hand1, p1hand2, p2hand1, p2hand2, turn): (i32, i32, i32, i32, i32)) {
    println!("          | Hand  1 | Hand  2");
    println!("------------------------------");
    println!("Player 1  |    {}    |    {}", p1hand1, p1hand2);
    println!("------------------------------");
    println!("Player 2  |    {}    |    {}", p2hand1, p2hand2);
    println!("------------------------------");
    println!("Turn Number: {}", turn);
    println!("------------------------------");
    println!()
}

pub fn get_turn_choice() -> TurnChoice {
    println!("Input 's' for split, or 'h' for hit: ");
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input.");

    match choice.trim().to_lowercase().as_str() {
        "s" => TurnChoice::Split,
        "h" => {
            let (src, dst) = get_hit_values();
            TurnChoice::Hit(src, dst)
        },
        _ => {
            println!("{} is not a valid choice.", choice);
            get_turn_choice()
        }
    }
}

fn get_hit_values() -> (u8, u8) {
    let src_hand: u8;
    let dst_hand: u8;

    loop {
        println!("Enter which hand to hit from: ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        src_hand = match input.trim().parse() {
            Ok(hand) => {
                match check_valid_hit_value(hand) {
                    true => hand,
                    false => {
                        println!("{} is not a valid hand number.", hand);
                        continue
                    }
                }
            },
            Err(_) => {
                println!("Must be a number.");
                continue
            }
        };
        break;
    }

    loop {
        println!("Enter which hand to hit: ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        dst_hand = match input.trim().parse() {
            Ok(hand) => {
                match check_valid_hit_value(hand) {
                    true => hand,
                    false => {
                        println!("{} is not a valid hand number.", hand);
                        continue
                    }
                }
            },
            Err(_) => {
                println!("Must be a number.");
                continue
            }
        };
        break;
    }
    (src_hand, dst_hand)
}

fn check_valid_hit_value(val: u8) -> bool {
    match val {
        1 => true,
        2 => true,
        _ => false
    }
}
