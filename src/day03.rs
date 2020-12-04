//! --- Day 3: No Matter How You Slice It ---
//!
//! The Elves managed to locate the chimney-squeeze prototype fabric for Santa's suit
//! (thanks to someone who helpfully wrote its box IDs on the wall of the warehouse in
//! the middle of the night). Unfortunately, anomalies are still affecting them -
//! nobody can even agree on how to cut the fabric.
//!
//! The whole piece of fabric they're working on is a very large square -
//! at least 1000 inches on each side.
//!
//! Each Elf has made a claim about which area of fabric would be ideal for Santa's suit.
//! All claims have an ID and consist of a single rectangle with edges parallel to the
//! edges of the fabric. Each claim's rectangle is defined as follows:
//!
//!     The number of inches between the left edge of the fabric and the left edge of the rectangle.
//!     The number of inches between the top edge of the fabric and the top edge of the rectangle.
//!     The width of the rectangle in inches.
//!     The height of the rectangle in inches.
//!
//! A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches
//! from the left edge, 2 inches from the top edge, 5 inches wide, and 4 inches tall.
//! Visually, it claims the square inches of fabric represented by #
//! (and ignores the square inches of fabric represented by .) in the diagram below:
//!
//! ...........
//! ...........
//! ...#####...
//! ...#####...
//! ...#####...
//! ...#####...
//! ...........
//! ...........
//! ...........
//!
//! The problem is that many of the claims overlap, causing two or more claims
//! to cover part of the same areas. For example, consider the following claims:
//!
//! #1 @ 1,3: 4x4
//! #2 @ 3,1: 4x4
//! #3 @ 5,5: 2x2
//!
//! Visually, these claim the following areas:
//!
//! ........
//! ...2222.
//! ...2222.
//! .11XX22.
//! .11XX22.
//! .111133.
//! .111133.
//! ........
//!
//! The four square inches marked with X are claimed by both 1 and 2.
//! (Claim 3, while adjacent to the others, does not overlap either of them.)
//!
//! If the Elves all proceed with their own plans, none of them will have enough fabric.
//! How many square inches of fabric are within two or more claims?
//!
//!
//! --- Part Two ---
//!
//! Amidst the chaos, you notice that exactly one claim doesn't overlap by even a
//! single square inch of fabric with any other claim. If you can somehow draw
//! attention to it, maybe the Elves will be able to make Santa's suit after all!
//!
//! For example, in the claims above, only claim 3 is intact after all claims are made.
//!
//! What is the ID of the only claim that doesn't overlap?

pub fn solve() -> crate::Result<()> {
    let input = crate::read_input("day03.txt")?;
    println!("Day03 part1: {}", part1(&input));
    println!("Day03 part2: {}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> usize {
    let fabrics: Vec<_> = parse(input).into_iter().collect();
    let width = 1000;
    let mut tiles = vec![0u8; width * 1000];

    fabrics.iter().for_each(|fab| {
        for y in fab.y..fab.y + fab.h {
            for x in fab.x..fab.x + fab.w {
                tiles[(y * width) + x] += 1;
            }
        }
    });

    tiles.iter().filter(|&&count| 1 < count).count()
}
// 149 totals
fn part2(input: &str) -> usize {
    let fabrics: Vec<_> = parse(input).into_iter().collect();

    fabrics
        .iter()
        .find(|&fab| {
            fabrics
                .iter()
                .filter(|&f2| fab.id != f2.id)
                .all(|f2| !fab.overlaps(f2))
        })
        .expect("All claims overlap")
        .id
}

fn parse<'a>(input: &'a str) -> impl IntoIterator<Item = Fabric> + Clone + 'a {
    input
        .trim()
        .lines()
        .map(|s| s.parse().expect("Parsing Fabric"))
}

#[derive(parse_display::FromStr)]
#[display("#{id} @ {x},{y}: {w}x{h}")]
struct Fabric {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Fabric {
    fn overlaps(&self, other: &Self) -> bool {
        !(self.x + self.w <= other.x
            || other.x + other.w <= self.x
            || self.y + self.h <= other.y
            || other.y + other.h <= self.y)
    }
}

#[cfg(test)]
mod tests {

    static INPUT: &str = r#"
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"#;

    #[test]
    fn part1_example() {
        assert_eq!(super::part1(INPUT.trim()), 4);
    }

    #[test]
    fn part1() {
        let input = crate::read_input("day03.txt").expect("Reading input");
        assert_eq!(super::part1(&input), 101469);
    }

    #[test]
    fn part2_example() {
        assert_eq!(super::part2(INPUT.trim()), 3);
    }

    #[test]
    fn part2() {
        let input = crate::read_input("day03.txt").expect("Reading input");
        assert_eq!(super::part2(input.trim()), 1067);
    }
}
