use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn make_random_sublist(word_count: usize, words: &[&str]) {
    debug_assert!(word_count <= words.len());
    let mut words = words.iter().map(|word| word.to_string()).collect::<Vec<_>>();
    words.shuffle(&mut thread_rng());
    let mut sublist = Vec::with_capacity(word_count);
    for i in 0..word_count {
        sublist.push(words[i].clone());
    }
    sublist.sort();
    println!("pub const WORDS: [&str; {}] = [", word_count);
    for word in sublist.iter() {
        println!("\t\"{}\",", word);
    }
    println!("];")
}

pub const WORDS_1: [&str; 10] = [
    "arranged",
    "classify",
    "connection",
    "escape",
    "include",
    "label",
    "sum",
    "suppose",
    "swoop",
    "vanish",
];

pub const WORDS_2: [&str; 2] = [
    "classify",
    "escape",
];

pub const WORDS_3: [&str; 2] = [
    "dog",
    "owner",
];

// One word has no shared characters with others.
pub const WORDS_4: [&str; 3] = [
    "dog",
    "owner",
    "lazy",
];

pub const ALL_SECOND_GRADE: [&str; 75] = [
    "amaze",
    "amusing",
    "analyze",
    "annoy",
    "arranged",
    "avoid",
    "cause",
    "classify",
    "community",
    "conclusion",
    "connection",
    "continue",
    "cooperation",
    "curious",
    "cycle",
    "data",
    "describe",
    "detail",
    "diagram",
    "difference",
    "different",
    "discover",
    "drowsy",
    "edit",
    "effect",
    "energy",
    "enormous",
    "escape",
    "estimate",
    "exercise",
    "expect",
    "famous",
    "flock",
    "friendly",
    "frighten",
    "frown",
    "gasp",
    "gather",
    "gust",
    "helpful",
    "include",
    "insist",
    "investigate",
    "label",
    "leaned",
    "living",
    "march",
    "matter",
    "moist",
    "necessary",
    "nonliving",
    "noticed",
    "observed",
    "opinion",
    "peeking",
    "plan",
    "poke",
    "predict",
    "prefer",
    "process",
    "publish",
    "records",
    "revise",
    "separate",
    "steaming",
    "shivered",
    "similar",
    "sum",
    "suppose",
    "sway",
    "stormy",
    "swoop",
    "treasure",
    "vanish",
    "volunteer",
];

