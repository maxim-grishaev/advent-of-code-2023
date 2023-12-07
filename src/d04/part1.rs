use super::help;

pub fn run() -> u32 {
    let cards = help::read_cards();
    cards
        .iter()
        .map(|card| help::intersection(&card.win, &card.have))
        .map(|match_nums| {
            let ln = match_nums.len();
            if ln == 0 {
                return 0;
            }
            // println!("Matches: {:?}", ln);
            2_u32.pow(ln as u32 - 1)
        })
        .sum()
}
