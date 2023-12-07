use super::help;

pub fn run() -> u32 {
    let cards = help::read_cards();
    let mut cards_acc: Vec<u32> = vec![1; cards.len()];
    for (i, card) in cards.into_iter().enumerate() {
        let won_count = help::intersection(&card.win, &card.have).len();
        let mul = cards_acc[i];
        for j in 0..won_count {
            cards_acc[i + j + 1] += mul;
        }
    }
    cards_acc.into_iter().sum()
}
