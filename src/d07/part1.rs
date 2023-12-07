use super::{help, poker};

pub const WEIGHTS_PLAIN: poker::CardsMap = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn get_char_weight(chr: &char) -> usize {
    WEIGHTS_PLAIN.into_iter().position(|x| &x == chr).unwrap()
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

fn compare_normally(a: &poker::GameItem, b: &poker::GameItem) -> std::cmp::Ordering {
    match poker::HandType::from(&a.card_stack).cmp(&poker::HandType::from(&b.card_stack)) {
        std::cmp::Ordering::Equal => compare_cards_stack_normally(&a.card_stack, &b.card_stack),
        ord => ord,
    }
}

fn get_it(str: &str) -> u128 {
    let mut games = help::parse_poker_string(str);
    games.sort_by(compare_normally);
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
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run(), 245794640);
    }

    #[test]
    fn test_run_2() {
        // (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5) = 6440
        assert_eq!(get_it(&help::t::get_test()), 6440);
    }
}
