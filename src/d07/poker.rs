use std::{collections::HashMap, fmt::Debug};

pub type CardsMap = [char; 13];
pub type Bid = u16;

pub struct Card {
    pub chr: char,
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card {}", &self.chr)
    }
}

impl Card {
    pub fn new(chr: char) -> Self {
        Card { chr }
    }
}

#[derive(Debug)]
pub struct CardsStack {
    pub stack: [Card; 5],
}

impl CardsStack {
    pub fn new(cards_str: &str) -> Self {
        let crd = cards_str.chars().map(Card::new).collect::<Vec<Card>>();
        CardsStack {
            stack: crd.try_into().unwrap(),
        }
    }
    pub fn count_cards(&self) -> Vec<(char, u8)> {
        let mut hm: HashMap<char, u8> = HashMap::new();
        self.stack.iter().for_each(|c| {
            let counter = hm.entry(c.chr).or_insert(0);
            *counter += 1;
        });
        let mut pairs: Vec<(char, u8)> = hm.into_iter().collect();
        pairs.sort_by(|a, b| a.1.cmp(&b.1));
        pairs
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPairs = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    pub fn from_sorted_count(counts: Vec<&u8>) -> Self {
        match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPairs,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Unexpected card stack: {:?}", counts),
        }
    }
    pub fn from_sorted_pairs(pairs: Vec<(char, u8)>) -> Self {
        let counts = pairs
            .iter()
            .map(|(_chr, count)| count)
            .collect::<Vec<&u8>>();
        HandType::from_sorted_count(counts)
    }
}

impl From<&CardsStack> for HandType {
    fn from(card_stack: &CardsStack) -> Self {
        HandType::from_sorted_pairs(card_stack.count_cards())
    }
}

#[derive(Debug)]
pub struct GameItem {
    pub card_stack: CardsStack,
    pub bid: Bid,
}

impl GameItem {
    pub fn new(s: &str) -> Self {
        let mut line_split = s.trim().split_whitespace();

        let cards_str = line_split.next().unwrap();
        let card_stack = CardsStack::new(cards_str);
        let bid = line_split.next().unwrap().parse::<Bid>().unwrap();
        GameItem { card_stack, bid }
    }
}

#[cfg(test)]
mod tests {
    use super::super::help;
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn smoke_test() {
        let games = help::t::get_test()
            .lines()
            .map(|l| GameItem::new(l))
            .collect::<Vec<GameItem>>();
        assert_debug_snapshot!(games);
    }
}
