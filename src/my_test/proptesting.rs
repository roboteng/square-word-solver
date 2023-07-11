use crate::{double_sided::DoubleSidedFinderMT, First, SolutionFinder};

use proptest::{prelude::*, sample::subsequence};

const WORDS: &str = include_str!("../../words.txt");

proptest! {
    #[ignore = "costly"]
    #[test]
    fn s(k in my_words()) {
        let a = DoubleSidedFinderMT::new(&k);
        let b = First::new(&k);

        let a = a.find();
        let b = b.find();

        if a.len() != b.len() {
            println!{"double: {a:?}"};
            println!("first: {b:?}")
        }

        assert!(a.len() == b.len())
    }

    #[ignore = "not currently needed"]
    #[test]
    fn diferent(k in short_words()) {

        let first = {
            let l = First::new(&k);
            l.find()
        };
        let double = {
            let l = DoubleSidedFinderMT::new(&k);
            l.find()
        };
        assert_eq!(first, double);
    }

}

fn my_words() -> impl Strategy<Value = Vec<&'static str>> {
    let words: Vec<&str> = WORDS.lines().take(2000).collect();
    let len = words.len();
    subsequence(words, 0..len)
}

fn short_words() -> impl Strategy<Value = Vec<&'static str>> {
    let words: Vec<&str> = vec![
        "event", "clues", "angel", "scent", "towel", "souls", "elect", "buggy", "pumps", "loans",
        "spins", "files", "oxide", "pains", "photo", "rival", "flats", "syrup", "rodeo", "sands",
        "moose", "pints", "curly", "cloak", "onion", "clams", "scrap", "didst", "couch", "codes",
        "fails", "lodge", "greet", "gypsy", "utter", "paved", "zones", "fours", "alley", "tiles",
        "bless", "crest", "elder", "kills", "yeast", "erect", "bugle", "medal", "roles", "hound",
        "snail", "ankle", "relay", "loops", "bites", "modes", "debts", "realm", "glove", "rayon",
        "poked", "stray", "lumps", "graze", "dread", "barns", "docks", "masts", "pours", "wharf",
        "curse", "plump", "robes", "seeks", "cedar", "jolly", "myths", "cages", "locks", "pedal",
        "beets", "crows", "anode", "slash", "creep", "rowed", "chips", "fists", "wines", "cares",
        "motel", "ivory", "necks", "barge", "blues", "alien", "frown", "strap", "crews", "shack",
        "gonna", "saves", "stump", "ferry", "idols", "cooks", "juicy", "glare", "carts", "alloy",
        "bulbs", "lawns", "lasts", "fuels", "oddly", "crane", "filed", "weird", "shawl", "slips",
        "troop", "suite", "sleek", "quilt", "tramp", "blaze", "atlas", "odors", "scrub", "crabs",
        "probe", "logic", "adobe", "exile", "rebel", "grind", "sting", "spine", "cling", "desks",
        "grove", "leaps", "prose", "lofty", "agony", "snare", "tusks", "bulls", "moods", "humid",
        "finer", "dimly", "plank", "china", "pines", "guilt", "sacks", "brace", "quote", "lathe",
        "gaily", "fonts", "scalp", "foggy", "ferns", "grams", "clump", "perch", "tumor", "teens",
        "crank", "fable", "hedge", "genes", "sober", "boast", "tract", "cigar", "unite", "owing",
        "haiku", "swish", "dikes", "wedge", "booth", "eased", "frail", "cough", "tombs", "darts",
        "forts", "choir", "pouch", "pinch", "hairy", "buyer", "torch", "vigor", "heats", "herbs",
        "users", "flint", "click", "madam", "bleak", "blunt", "aided", "lacks", "masks", "waded",
        "risks", "nurse", "chaos", "cured", "ample", "lease", "steak", "sinks", "merit", "bluff",
        "bathe", "gleam", "bonus", "shear", "gland", "silky", "skate", "anvil", "sleds", "groan",
        "maids", "meets", "speck", "hymns", "hints", "drown", "slick", "quest", "coils", "snows",
        "snack", "plows", "blond", "tamed", "thorn", "waits", "glued", "banjo", "arena", "bulky",
        "stunt", "warms", "shady", "razor", "folly", "leafy", "notch", "fools", "otter", "pears",
        "flush", "genus", "ached", "fives", "flaps", "spout", "smote", "cuffs", "tasty", "stoop",
        "clips", "disks", "sniff", "lanes", "imply", "demon", "super", "furry", "raged", "growl",
        "texts", "hardy", "stung", "typed", "hates", "wiser", "serum", "beaks", "rotor", "casts",
        "baths", "glide", "plots", "trait", "slums", "lyric", "puffs", "decks", "brood", "mourn",
        "aloft", "abuse", "whirl", "edged", "ovary", "quack", "heaps", "slang", "await", "civic",
        "saint", "bevel", "sonar", "aunts", "packs", "froze", "tonic", "corps", "swarm", "frank",
        "repay", "gaunt", "wired", "niece", "cello", "needy", "chuck", "stony", "media", "surge",
        "hurts", "repel", "husky", "dated", "hunts", "mists", "exert", "dries", "mates", "sworn",
        "spice", "oasis", "boils", "spurs", "doves", "sneak", "paces", "colon", "siege", "strum",
        "drier", "cacao", "humus", "bales", "piped", "nasty", "rinse", "boxer", "shrub", "amuse",
        "tacks", "cited", "laden", "larva", "rents", "yells", "spool", "crush", "jewel", "snaps",
        "stain", "kicks", "tying", "slits", "rated", "eerie", "smash", "zebra", "bushy", "scary",
        "squad", "tutor", "silks", "slabs", "evils", "fangs", "snout", "peril", "yacht", "lobby",
        "jeans", "grins", "viola", "liner", "scars", "chops", "raids", "eater", "slate", "skips",
        "soles", "misty", "urine", "knobs", "sleet", "holly", "pests", "forks", "grill", "borne",
        "carol", "woody", "canon", "wakes", "kitty", "miner", "polls", "nasal", "scorn", "chess",
        "taxis", "crate", "shyly", "tulip", "forge", "nymph", "budge", "abide", "depot", "oases",
        "asses", "sheds", "fudge", "pills", "rivet", "thine", "groom", "lanky", "boost", "broth",
        "gravy", "beech", "timed", "quail", "inert", "gears", "chick", "hinge", "trash", "clash",
        "sighs", "renew", "bough", "dwarf", "slows", "quill", "shave", "spore", "sixes", "chunk",
        "madly", "paced", "braid", "fuzzy", "motto", "spies", "slack", "mucus", "magma", "awful",
        "discs", "erase", "posed", "asset", "cider", "taper", "theft", "churn", "satin", "taxed",
        "sloth", "shale", "tread", "raked", "curds", "manor", "bulge", "loins", "stair", "leans",
        "bunks", "squat", "towed", "lance", "panes", "sakes", "heirs", "caste", "dummy", "pores",
        "fauna", "crook", "poise", "epoch", "risky", "warns", "fling", "berry", "grape", "flank",
        "drags", "squid", "pelts", "icing", "irony", "whoop", "choke", "diets", "whips", "tally",
        "dozed", "twine", "kites", "bikes", "ticks", "riots", "roars", "vault", "looms", "blink",
        "dandy", "pupae", "sieve", "spike", "ducts", "lends", "pizza", "brink", "plumb", "pagan",
        "feats", "bison", "soggy", "scoop", "argon", "nudge", "amber", "sexes", "salts", "hitch",
        "exalt", "leash", "dined", "chute", "snort", "gusts", "cheat", "llama", "lasso", "debut",
        "quota", "oaths", "prone", "mixes", "rafts", "dives", "stale", "inlet", "flick", "pinto",
        "brows", "untie", "greed", "stirs", "blush", "barbs", "volts", "beige", "swoop", "paddy",
        "shove", "jerky", "poppy", "leaks", "fares", "dodge", "godly", "squaw", "affix", "brute",
        "nicer", "undue", "snarl", "merge", "doses", "showy", "daddy", "roost", "vases", "swirl",
        "petty", "colds", "cobra", "genie", "flare", "messy", "cores", "soaks", "whine", "amino",
        "plaid", "baton", "peers", "vowed", "pious", "swans", "exits", "afoot", "plugs", "idiom",
        "chili", "rites", "serfs", "berth", "grubs", "annex", "dizzy", "hasty", "latch", "mirth",
        "baron", "plead", "aging", "pixel", "mummy", "hotly", "auger", "buddy", "chaps", "badge",
        "stark", "fairs", "gully", "mumps", "emery", "filly", "ovens", "drone", "gauze", "idiot",
        "fussy", "shank", "gouge", "elves", "roped", "unfit", "baggy", "mower", "scant", "grabs",
        "fleas", "lousy", "album", "sawed", "cooky", "murky", "infer", "burly", "waged", "dingy",
        "brine", "kneel", "creak", "vanes", "smoky", "spurt", "combs", "easel", "laces", "humps",
        "rumor", "horde", "swiss", "leapt", "opium", "slime", "afire", "pansy", "mares", "husks",
        "snips", "hazel", "lined", "naive", "wraps", "sized", "piers", "beset", "agile", "steed",
        "fraud", "booty", "valor", "downy", "witty", "mossy", "psalm", "scuba", "tours", "polka",
        "milky", "gaudy", "shrug", "tufts", "wilds", "laser", "truss", "hares", "creed", "lilac",
        "siren", "tarry", "bribe", "swine", "muted", "flips", "cures", "sinew", "boxed", "hoops",
        "gasps", "hoods", "niche", "yucca", "glows", "sewer", "whack", "fuses", "gowns", "droop",
        "pangs", "mails", "whisk", "haven", "clasp", "sling", "stint", "urges", "champ", "piety",
        "pleat", "posse", "sunup", "menus", "howls", "quake", "knack", "plaza", "fiend", "caked",
        "bangs", "poker", "olden", "cramp", "voter", "poses", "fined", "grips", "gaped", "purge",
        "hiked", "maize", "fluff", "strut", "sloop", "prowl", "roach", "cocks", "bland", "dials",
        "plume", "slaps", "soups", "foams", "solos", "skier", "eaves", "totem", "fused", "latex",
        "mused", "mains", "myrrh", "galls",
    ];
    let len = words.len();
    subsequence(words, 0..len)
}
