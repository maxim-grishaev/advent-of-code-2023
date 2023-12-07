use super::{help, poker};

pub const WEIGHTS_JOKER: poker::CardsMap = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn get_char_weight(chr: &char) -> usize {
    WEIGHTS_JOKER.into_iter().position(|x| &x == chr).unwrap()
}

fn compare_cards_stack_normally(
    a: &poker::CardsStack,
    b: &poker::CardsStack,
) -> std::cmp::Ordering {
    for (i, c) in a.stack.iter().enumerate() {
        let c_weight = get_char_weight(&c.chr);
        let o_weight = get_char_weight(&b.stack[i].chr);
        if c_weight > o_weight {
            return std::cmp::Ordering::Greater;
        }
        if c_weight < o_weight {
            return std::cmp::Ordering::Less;
        }
    }
    std::cmp::Ordering::Equal
}

const JOKER: char = 'J';
fn is_joker(c: char) -> bool {
    c == JOKER
}
type Pos = (char, u8);
fn is_joker_pos(c: &Pos) -> bool {
    is_joker(c.0)
}

fn get_best_hand_type(cards: &poker::CardsStack) -> poker::HandType {
    let counts = cards.count_cards();
    // Any 5, including 5 Jokers
    if counts.len() == 1 {
        return poker::HandType::FiveOfAKind;
    }

    let joker_pos: Option<usize> = counts.iter().position(is_joker_pos);

    // No Jokers
    if joker_pos.is_none() {
        return poker::HandType::from(cards);
    }

    // Some Jokers
    let j_count = counts[joker_pos.unwrap()].1;

    let counts_clone = counts.clone();
    // remove Jokers
    let mut new_cards = counts_clone
        .into_iter()
        .filter(|c| !is_joker_pos(c))
        .collect::<Vec<_>>();

    // add Joker to the "best" card
    let las_idx = new_cards.len() - 1;
    let last = new_cards[las_idx];
    new_cards[las_idx] = (last.0, last.1 + j_count);

    poker::HandType::from_sorted_pairs(new_cards)
}

fn compare_cards_stack_with_joker(a: &poker::GameItem, b: &poker::GameItem) -> std::cmp::Ordering {
    let bh_a = get_best_hand_type(&a.card_stack);
    let bh_b = get_best_hand_type(&b.card_stack);
    match bh_a.cmp(&bh_b) {
        std::cmp::Ordering::Equal => compare_cards_stack_normally(&a.card_stack, &b.card_stack),
        ord => ord,
    }
}

fn get_it(str: &str) -> u128 {
    let mut games = help::parse_poker_string(str);
    games.sort_by(compare_cards_stack_with_joker);
    games
        .iter()
        .enumerate()
        .map(|(i, g)| g.bid as u128 * (i as u128 + 1))
        .sum()
}

pub fn run() -> u128 {
    get_it(&help::read_poker_string())
}

#[cfg(test)]
mod tests {
    // use insta::assert_debug_snapshot;
    use super::*;

    #[test]
    fn real_run() {
        assert_eq!(run(), 247899149);
    }

    #[test]
    fn test_run() {
        assert_eq!(get_it(&help::t::get_test()), 5905);
    }
}
