#![feature(test)]
#![feature(iter_intersperse)]
#![feature(array_zip)]
extern crate num_cpus;
use ascii::{AsciiChar, AsciiStr, AsciiString};
use builder::AddedWord;
use finder::{Puzzle, PuzzleViewModel};
use regex::Regex;
use std::io;
use std::str::FromStr;
use std::{collections::HashMap, fmt::Display, fs::File, io::Read, path::Path};

mod builder;
pub mod double_sided;
pub mod finder;
pub mod first_guess;
pub mod solver;
pub mod top_down_finder;
pub mod trivial_finder;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Word([AsciiChar; 5]);

impl From<&str> for Word {
    fn from(value: &str) -> Self {
        let k = value
            .chars()
            .map(|c| AsciiChar::from_ascii(c).unwrap())
            .collect::<Vec<AsciiChar>>();
        Self(k.try_into().unwrap())
    }
}

impl From<String> for Word {
    fn from(value: String) -> Self {
        let k = value
            .chars()
            .map(|c| AsciiChar::from_ascii(c).unwrap())
            .collect::<Vec<AsciiChar>>();
        Self(k.try_into().unwrap())
    }
}

impl From<AsciiString> for Word {
    fn from(value: AsciiString) -> Self {
        Self(value.chars().collect::<Vec<_>>().try_into().unwrap())
    }
}

impl From<&AsciiStr> for Word {
    fn from(value: &AsciiStr) -> Self {
        Self(value.chars().collect::<Vec<_>>().try_into().unwrap())
    }
}

impl From<Vec<AsciiChar>> for Word {
    fn from(value: Vec<AsciiChar>) -> Self {
        Self(value.try_into().unwrap())
    }
}

impl From<Word> for String {
    fn from(value: Word) -> Self {
        String::from_iter(value.0.iter().map(|ch| ch.as_char()))
    }
}

pub trait SolutionFinder<'a> {
    fn new(words: &'a [&'a str]) -> Self;
    fn find(&self) -> Vec<Solution>;
}

pub trait RangeFinder<'a> {
    fn init(words: &'a [Word]) -> Self;
    fn range(&self, new_word: &[AsciiChar]) -> std::ops::Range<usize>;
}

#[allow(dead_code)]
fn range_for(words: &[&str], new_word: &str) -> std::ops::Range<usize> {
    let start = words.partition_point(|word| word < &new_word);
    let end = words.partition_point(|word| word.starts_with(new_word) || word < &new_word);
    start..end
}

pub struct BinSearchRange(Vec<Word>);

impl<'a> RangeFinder<'a> for BinSearchRange {
    fn init(words: &[Word]) -> Self {
        Self(Vec::from(words))
    }

    fn range(&self, new_word: &[AsciiChar]) -> std::ops::Range<usize> {
        range_for_ascii(&self.0, new_word)
    }
}

fn range_for_ascii(words: &[Word], new_word: &[AsciiChar]) -> std::ops::Range<usize> {
    let start = words.partition_point(|word| word.0.as_slice() < new_word);
    let end = words.partition_point(|word| {
        word.0.as_slice().starts_with(new_word) || word.0.as_slice() < new_word
    });
    start..end
}

pub struct HasSearchRange(HashMap<AsciiString, std::ops::Range<usize>>);

impl<'a> RangeFinder<'a> for HasSearchRange {
    fn init(words: &'a [Word]) -> Self {
        let mut map = HashMap::new();
        for end in 1..5 {
            for word in words.iter() {
                let start = AsciiString::from(&word.0[0..end]);
                let start = start;
                let range = range_for_ascii(&words, start.as_slice());
                map.insert(start, range);
            }
        }
        Self(map)
    }

    fn range(&self, new_word: &[AsciiChar]) -> std::ops::Range<usize> {
        self.0
            .get(&AsciiString::from(new_word))
            .unwrap_or(&(0..0))
            .clone()
    }
}

pub fn get_words() -> Result<Vec<String>, io::Error> {
    let path = Path::new("all_words.txt");
    let mut file = File::open(path)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(five_letter_words(&buffer))
}

pub fn five_letter_words(string: &str) -> Vec<String> {
    let reg = Regex::new("^[a-z]{5}$").unwrap();
    let words = string.lines();
    words
        .filter(|word| -> bool { reg.is_match(word) })
        .map(|s| s.to_string())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Solution {
    pub rows: [Word; 5],
}

impl Solution {
    pub fn new<S: Into<Word>>(rows: [S; 5]) -> Self {
        Self {
            rows: rows.map(|s| s.into()),
        }
    }

    pub fn does_match(&self, view: &PuzzleViewModel) -> bool {
        let my_view = {
            let mut puzzle = Puzzle::new(self.clone());
            view.guesses.iter().for_each(|guess| {
                puzzle.guess(guess.clone());
            });
            puzzle.view()
        };

        my_view == *view
    }

    pub fn is_equivalent_to(&self, other: &PuzzleViewModel) -> bool {
        false
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .rows
                .iter()
                .map(|s| String::from(s.clone()))
                .intersperse(String::from(","))
                .collect::<String>(),
        )
    }
}

impl FromStr for Solution {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s
            .split(',')
            .filter_map(|w| AsciiString::from_ascii(w).ok())
            .collect::<Vec<AsciiString>>();
        if words.len() == 5 {
            Ok(Self {
                rows: words
                    .iter()
                    .map(|s| Word::from(s.clone()))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            })
        } else {
            Err(())
        }
    }
}

pub struct WordList {
    words: HashMap<char, Box<WordList>>,
}

impl WordList {
    pub fn new(words: Vec<&str>) -> WordList {
        let mut this = WordList {
            words: HashMap::new(),
        };
        for word in words.iter() {
            this.insert(word)
        }
        this
    }

    fn insert(&mut self, word: &str) {
        if let Some(first_letter) = word.chars().next() {
            let rest = &word[1..];
            self.words
                .entry(first_letter)
                .or_insert_with(|| {
                    Box::new(WordList {
                        words: HashMap::new(),
                    })
                })
                .insert(rest);
        }
    }

    pub fn contains(&self, word_to_check: &str) -> bool {
        let mut chars = word_to_check.chars();
        match chars.next() {
            Some(head) => match self.words.get(&head) {
                Some(dict) => dict.contains(chars.as_str()),
                None => false,
            },
            None => true,
        }
    }
}

#[cfg(test)]
mod my_test {
    use crate::{
        builder::SolutionBuilder,
        double_sided::DoubleSidedFinderMT,
        finder::Puzzle,
        top_down_finder::{find_subsolutions, TopDownFinder},
        trivial_finder::TrivialFinder,
    };
    use pretty_assertions::assert_eq;

    use super::*;
    use test::Bencher;
    extern crate test;

    #[test]
    fn empty_word_list_does_not_contain_a_word() {
        let l = WordList::new(vec![]);
        assert!(!l.contains("foo"));
    }

    #[test]
    fn word_list_contains_a_word() {
        let l = WordList::new(vec!["foo"]);
        assert!(l.contains("foo"));
    }

    #[test]
    fn word_list_does_not_contain_a_different_word() {
        let l = WordList::new(vec!["bar"]);
        assert!(!l.contains("foo"));
    }

    #[test]
    fn word_list_includes_if_the_starting_letters_match() {
        let l = WordList::new(vec!["foobar"]);
        assert!(l.contains("foo"));
    }

    #[test]
    fn is_able_to_two_words_that_start_with_the_same_letters() {
        let words = vec!["foo", "foobar"];
        let list = WordList::new(words);
        assert!(list.contains("foob"));
    }

    #[bench]
    fn dict_test(b: &mut Bencher) {
        let binding = get_words().unwrap();
        let words: Vec<&str> = binding.iter().map(|s| s.as_str()).collect();

        let list = WordList::new(words.clone());
        let first = words[0];
        let last = words[words.len() - 1];
        b.iter(|| {
            assert!(list.contains(first));
            assert!(list.contains(last));
            assert!(!list.contains("foobar"));
        })
    }

    #[bench]
    fn actual_solve(b: &mut Bencher) {
        let valid_words = vec![
            "grime", "honor", "outdo", "steed", "terse", "ghost", "route", "inter", "modes",
            "erode", "level", "oxide", "atria", "truck", "hasty", "loath", "extra", "virus",
            "edict", "leaky", "loses", "apple", "diode", "lured", "emery", "ladle", "opium",
            "spore", "elder", "seedy",
        ];
        let list = WordList::new(valid_words.clone());

        b.iter(|| find_subsolutions(&valid_words, &mut SolutionBuilder::new(&list)))
    }

    #[test]
    fn solution_should_match_vm_with_no_guesses() {
        let solution = Solution::new(["grime", "honor", "outdo", "steed", "terse"]);
        let puzzle = Puzzle::new(solution.clone());
        let view = puzzle.view();

        let actual = solution.does_match(&view);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn does_not_pass_when_grid_does_not_match() {
        let tester = Solution::new(["small", "movie", "irate", "loser", "entry"]);
        let answer = Solution::new(["small", "movie", "alive", "stark", "hones"]);
        let mut puzzle = Puzzle::new(answer);
        puzzle.guess(AsciiString::from_ascii("ricky").unwrap().into());
        let view = puzzle.view();

        let actual = tester.does_match(&view);
        assert_eq!(actual, false);
    }

    #[test]
    fn range_a_b_c_for_b() {
        let list = ["a", "b", "c"];
        let range = range_for(&list, "b");
        assert_eq!(range, 1..2)
    }

    #[test]
    fn big_range() {
        let list = ["about", "other", "their", "there", "these", "would"];
        let range = range_for(&list, "the");
        assert_eq!(range, 2..5)
    }

    #[ignore = "costly"]
    #[test]
    fn diferent() {
        let words = [
            "event", "clues", "angel", "scent", "towel", "souls", "elect", "buggy", "pumps",
            "loans", "spins", "files", "oxide", "pains", "photo", "rival", "flats", "syrup",
            "rodeo", "sands", "moose", "pints", "curly", "cloak", "onion", "clams", "scrap",
            "didst", "couch", "codes", "fails", "lodge", "greet", "gypsy", "utter", "paved",
            "zones", "fours", "alley", "tiles", "bless", "crest", "elder", "kills", "yeast",
            "erect", "bugle", "medal", "roles", "hound", "snail", "ankle", "relay", "loops",
            "bites", "modes", "debts", "realm", "glove", "rayon", "poked", "stray", "lumps",
            "graze", "dread", "barns", "docks", "masts", "pours", "wharf", "curse", "plump",
            "robes", "seeks", "cedar", "jolly", "myths", "cages", "locks", "pedal", "beets",
            "crows", "anode", "slash", "creep", "rowed", "chips", "fists", "wines", "cares",
            "motel", "ivory", "necks", "barge", "blues", "alien", "frown", "strap", "crews",
            "shack", "gonna", "saves", "stump", "ferry", "idols", "cooks", "juicy", "glare",
            "carts", "alloy", "bulbs", "lawns", "lasts", "fuels", "oddly", "crane", "filed",
            "weird", "shawl", "slips", "troop", "suite", "sleek", "quilt", "tramp", "blaze",
            "atlas", "odors", "scrub", "crabs", "probe", "logic", "adobe", "exile", "rebel",
            "grind", "sting", "spine", "cling", "desks", "grove", "leaps", "prose", "lofty",
            "agony", "snare", "tusks", "bulls", "moods", "humid", "finer", "dimly", "plank",
            "china", "pines", "guilt", "sacks", "brace", "quote", "lathe", "gaily", "fonts",
            "scalp", "foggy", "ferns", "grams", "clump", "perch", "tumor", "teens", "crank",
            "fable", "hedge", "genes", "sober", "boast", "tract", "cigar", "unite", "owing",
            "haiku", "swish", "dikes", "wedge", "booth", "eased", "frail", "cough", "tombs",
            "darts", "forts", "choir", "pouch", "pinch", "hairy", "buyer", "torch", "vigor",
            "heats", "herbs", "users", "flint", "click", "madam", "bleak", "blunt", "aided",
            "lacks", "masks", "waded", "risks", "nurse", "chaos", "cured", "ample", "lease",
            "steak", "sinks", "merit", "bluff", "bathe", "gleam", "bonus", "shear", "gland",
            "silky", "skate", "anvil", "sleds", "groan", "maids", "meets", "speck", "hymns",
            "hints", "drown", "slick", "quest", "coils", "snows", "snack", "plows", "blond",
            "tamed", "thorn", "waits", "glued", "banjo", "arena", "bulky", "stunt", "warms",
            "shady", "razor", "folly", "leafy", "notch", "fools", "otter", "pears", "flush",
            "genus", "ached", "fives", "flaps", "spout", "smote", "cuffs", "tasty", "stoop",
            "clips", "disks", "sniff", "lanes", "imply", "demon", "super", "furry", "raged",
            "growl", "texts", "hardy", "stung", "typed", "hates", "wiser", "serum", "beaks",
            "rotor", "casts", "baths", "glide", "plots", "trait", "slums", "lyric", "puffs",
            "decks", "brood", "mourn", "aloft", "abuse", "whirl", "edged", "ovary", "quack",
            "heaps", "slang", "await", "civic", "saint", "bevel", "sonar", "aunts", "packs",
            "froze", "tonic", "corps", "swarm", "frank", "repay", "gaunt", "wired", "niece",
            "cello", "needy", "chuck", "stony", "media", "surge", "hurts", "repel", "husky",
            "dated", "hunts", "mists", "exert", "dries", "mates", "sworn", "spice", "oasis",
            "boils", "spurs", "doves", "sneak", "paces", "colon", "siege", "strum", "drier",
            "cacao", "humus", "bales", "piped", "nasty", "rinse", "boxer", "shrub", "amuse",
            "tacks", "cited", "laden", "larva", "rents", "yells", "spool", "crush", "jewel",
            "snaps", "stain", "kicks", "tying", "slits", "rated", "eerie", "smash", "zebra",
            "bushy", "scary", "squad", "tutor", "silks", "slabs", "evils", "fangs", "snout",
            "peril", "yacht", "lobby", "jeans", "grins", "viola", "liner", "scars", "chops",
            "raids", "eater", "slate", "skips", "soles", "misty", "urine", "knobs", "sleet",
            "holly", "pests", "forks", "grill", "borne", "carol", "woody", "canon", "wakes",
            "kitty", "miner", "polls", "nasal", "scorn", "chess", "taxis", "crate", "shyly",
            "tulip", "forge", "nymph", "budge", "abide", "depot", "oases", "asses", "sheds",
            "fudge", "pills", "rivet", "thine", "groom", "lanky", "boost", "broth", "gravy",
            "beech", "timed", "quail", "inert", "gears", "chick", "hinge", "trash", "clash",
            "sighs", "renew", "bough", "dwarf", "slows", "quill", "shave", "spore", "sixes",
            "chunk", "madly", "paced", "braid", "fuzzy", "motto", "spies", "slack", "mucus",
            "magma", "awful", "discs", "erase", "posed", "asset", "cider", "taper", "theft",
            "churn", "satin", "taxed", "sloth", "shale", "tread", "raked", "curds", "manor",
            "bulge", "loins", "stair", "leans", "bunks", "squat", "towed", "lance", "panes",
            "sakes", "heirs", "caste", "dummy", "pores", "fauna", "crook", "poise", "epoch",
            "risky", "warns", "fling", "berry", "grape", "flank", "drags", "squid", "pelts",
            "icing", "irony", "whoop", "choke", "diets", "whips", "tally", "dozed", "twine",
            "kites", "bikes", "ticks", "riots", "roars", "vault", "looms", "blink", "dandy",
            "pupae", "sieve", "spike", "ducts", "lends", "pizza", "brink", "plumb", "pagan",
            "feats", "bison", "soggy", "scoop", "argon", "nudge", "amber", "sexes", "salts",
            "hitch", "exalt", "leash", "dined", "chute", "snort", "gusts", "cheat", "llama",
            "lasso", "debut", "quota", "oaths", "prone", "mixes", "rafts", "dives", "stale",
            "inlet", "flick", "pinto", "brows", "untie", "greed", "stirs", "blush", "barbs",
            "volts", "beige", "swoop", "paddy", "shove", "jerky", "poppy", "leaks", "fares",
            "dodge", "godly", "squaw", "affix", "brute", "nicer", "undue", "snarl", "merge",
            "doses", "showy", "daddy", "roost", "vases", "swirl", "petty", "colds", "cobra",
            "genie", "flare", "messy", "cores", "soaks", "whine", "amino", "plaid", "baton",
            "peers", "vowed", "pious", "swans", "exits", "afoot", "plugs", "idiom", "chili",
            "rites", "serfs", "berth", "grubs", "annex", "dizzy", "hasty", "latch", "mirth",
            "baron", "plead", "aging", "pixel", "mummy", "hotly", "auger", "buddy", "chaps",
            "badge", "stark", "fairs", "gully", "mumps", "emery", "filly", "ovens", "drone",
            "gauze", "idiot", "fussy", "shank", "gouge", "elves", "roped", "unfit", "baggy",
            "mower", "scant", "grabs", "fleas", "lousy", "album", "sawed", "cooky", "murky",
            "infer", "burly", "waged", "dingy", "brine", "kneel", "creak", "vanes", "smoky",
            "spurt", "combs", "easel", "laces", "humps", "rumor", "horde", "swiss", "leapt",
            "opium", "slime", "afire", "pansy", "mares", "husks", "snips", "hazel", "lined",
            "naive", "wraps", "sized", "piers", "beset", "agile", "steed", "fraud", "booty",
            "valor", "downy", "witty", "mossy", "psalm", "scuba", "tours", "polka", "milky",
            "gaudy", "shrug", "tufts", "wilds", "laser", "truss", "hares", "creed", "lilac",
            "siren", "tarry", "bribe", "swine", "muted", "flips", "cures", "sinew", "boxed",
            "hoops", "gasps", "hoods", "niche", "yucca", "glows", "sewer", "whack", "fuses",
            "gowns", "droop", "pangs", "mails", "whisk", "haven", "clasp", "sling", "stint",
            "urges", "champ", "piety", "pleat", "posse", "sunup", "menus", "howls", "quake",
            "knack", "plaza", "fiend", "caked", "bangs", "poker", "olden", "cramp", "voter",
            "poses", "fined", "grips", "gaped", "purge", "hiked", "maize", "fluff", "strut",
            "sloop", "prowl", "roach", "cocks", "bland", "dials", "plume", "slaps", "soups",
            "foams", "solos", "skier", "eaves", "totem", "fused", "latex", "mused", "mains",
            "myrrh", "galls",
        ];

        let first = {
            let k = TopDownFinder::new(&words);
            k.find()
        };
        let double = {
            let k = DoubleSidedFinderMT::<BinSearchRange>::new(&words);
            k.find()
        };
        assert_eq!(first, double);
    }

    #[test]
    fn double_minimal() {
        let words = [
            "event", "clues", "angel", "scent", "larva", "pests", "lance", "pelts", "salts",
            "clasp", "urges",
        ];

        let double = {
            let k = DoubleSidedFinderMT::<BinSearchRange>::new(&words);
            let mut sol = k.find();
            sol.sort();
            sol
        };
        let known = {
            let k = TrivialFinder::new(&words);
            let mut sol = k.find();
            sol.sort();
            sol
        };
        assert_eq!(double, known);
    }

    #[ignore = "fix later"]
    #[test]
    fn first_minimal() {
        let words = [
            "event", "clues", "angel", "scent", "larva", "pests", "lance", "salts", "clasp",
            "urges", // extra word
            "pelts",
        ];

        let first = {
            let k = TopDownFinder::new(&words);
            let mut sol = k.find();
            sol.sort();
            sol
        };
        let known = {
            let k = TrivialFinder::new(&words);
            let mut sol = k.find();
            sol.sort();
            sol
        };
        assert_eq!(first, known);
    }

    #[test]
    fn word_list_fail() {
        let words = vec![
            "event", "clues", "angel", "scent", "larva", "pests", "lance", "salts", "clasp",
            "urges", // extra word
            "pelts",
        ];
        let list = WordList::new(words.clone());

        for len in 1..5 {
            for word in words.iter() {
                let sub_str = &word[0..len];
                assert!(
                    list.contains(sub_str),
                    "\"{sub_str}\" not found in list from word: {word}"
                )
            }
        }
    }

    mod proptesting;
}
