mod d00;
mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
#[path = "util/map2d.rs"]
mod map2d;
mod util;

fn main() {
    let prev_days = [
        || {
            println!(
                "Day 01: 1. {:?} 2. {:?}",
                d01::part1::run(),
                d01::part2::run()
            )
        },
        || {
            println!(
                "Day 02: 1. {:?} 2. {:?}",
                d02::part1::run(),
                d02::part2::run()
            )
        },
        || {
            println!(
                "Day 03: 1. {:?} 2. {:?}",
                d03::part1::run(),
                d03::part2::run()
            )
        },
        || {
            println!(
                "Day 04: 1. {:?} 2. {:?}",
                d04::part1::run(),
                d04::part2::run()
            )
        },
        || {
            println!(
                "Day 05: 1. {:?} 2. {:?}",
                d05::part1::run(),
                d05::part2::run()
            )
        },
        || {
            println!(
                "Day 06: 1. {:?} 2. {:?}",
                d06::part1::run(),
                d06::part2::run()
            )
        },
        || {
            println!(
                "Day 07: 1. {:?} 2. {:?}",
                d07::part1::run(),
                d07::part2::run()
            )
        },
        || {
            println!(
                "Day 08: 1. {:?} 2. {:?}",
                d08::part1::run(),
                d08::part2::run()
            )
        },
        || {
            println!(
                "Day 09: 1. {:?} 2. {:?}",
                d09::part1::run(),
                d09::part2::run()
            )
        },
        || {
            println!(
                "Day 10: 1. {:?} 2. {:?}",
                d10::part1::run(),
                d10::part2::run()
            )
        },
        || {
            println!(
                "Day 11: 1. {:?} 2. {:?}",
                d11::part1::run(),
                d11::part2::run()
            )
        },
        || {
            println!(
                "Day 12: 1. {:?} 2. {:?}",
                d12::part1::run(),
                d12::part2::run()
            )
        },
    ];
    prev_days.last().unwrap()();
}
