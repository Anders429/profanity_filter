use once_cell::sync::Lazy;
use word_filter::{Options, WordFilter};

pub static PROFANITY_FILTER: Lazy<WordFilter<'static>> = Lazy::new(|| WordFilter::new(
    // Filtered words.
    &[
        // Probably the best source on the web for these words:
        // https://www.reddit.com/r/copypasta/comments/fca22g/every_swear_word_in_alphabetical_order/

        // Profane.
        "dammit",
        "damn",
        "damnit",
        "goddammit",
        "goddamn",
        "goddamnit",
        // Vulgar.
        "anal",
        "analplug",
        "analplugs",
        "ass",
        "asses",
        "asshole",
        "blowjob",
        "bullshit",
        "cock",
        "cocks",
        "cocksucker",
        "cocksuckers",
        "cocksucking",
        "cum",
        "cummed",
        "cumming",
        "cums",
        "cumshot",
        "cunt",
        "dick",
        "dickhead",
        "dumbass",
        "dumbasses",
        "fuck",
        "fucked",
        "fucker",
        "fuckers",
        "fucking",
        "fucks",
        "jackass",
        "jackasses",
        "jackassery",
        "jerkoff",
        "jerkedoff",
        "jerkingoff",
        "jizz",
        "jizzing",
        "motherfuck",
        "motherfucker",
        "motherfucking",
        "pussy",
        "sex",
        "shit",
        "shits",
        "shittier",
        "shittiest",
        "shitting",
        "shitty",
        // Offensive.
        "bastard",
        "bastards",
        "bitch",
        "bitched",
        "bitches",
        "bitching",
        "chink",
        "coon",
        "nigger",
        "niggers",
        "slut",
        "sluts",
        // Emoji.
        "🖕",
    ],
    // Exceptions.
    &[
        // anal
        "analcime",
        "analcimic",
        "analcite",
        "analect",
        "analemma",
        "analeptic",
        "analgesia",
        "analgesic",
        "analgetic",
        "analgia",
        "analog",
        "analphabet",
        "analysand",
        "analyse",
        "analysing",
        "analysis",
        "analyst",
        "analyte",
        "analytic",
        "analyzabilities",
        "analyzability",
        "analyzable",
        "analyzation",
        "analyze",
        "analyzing",
        "artisanal",
        "bacchanal",
        "banal",
        "canal",
        "membranal",
        "olecranal",
        "tetanal",
        "tympanal",
        // ass
        "antimacassar",
        "as s",
        "assai",
        "assassin",
        "assault",
        "assay",
        "assegai",
        "assemblage",
        "assemblagist",
        "assemble",
        "assemblies",
        "assembling",
        "assembly",
        "assent",
        "assert",
        "assess",
        "asset",
        "asseverate",
        "asseverating",
        "asseveration",
        "asseverative",
        "assiduities",
        "assiduity",
        "assiduous",
        "assign",
        "assimilability",
        "assimilable",
        "assimilate",
        "assimilating",
        "assimilation",
        "assimilative",
        "assimilator",
        "assist",
        "assize",
        "associate",
        "associating",
        "association",
        "associative",
        "associativities",
        "associativity",
        "assoil",
        "assonance",
        "assort",
        "assuage",
        "assuasive",
        "assumabilities",
        "assumability",
        "assume",
        "assuming",
        "assumpsit",
        "assumption",
        "assumptive",
        "assurance",
        "assure",
        "assurgent",
        "assuring",
        "assuror",
        "asswage",
        "bagasse",
        "bass",
        "biassed",
        "biasses",
        "brass",
        "canvass",
        "carcass",
        "carnassial",
        "cassaba",
        "cassata",
        "cassation",
        "cassava",
        "cassette",
        "cassena",
        "cassene",
        "casserole",
        "cassia",
        "cassimere",
        "cassine",
        "cassingle",
        "cassino",
        "cassina",
        "cassis",
        "cassiterite",
        "cassock",
        "cassoulet",
        "cassowaries",
        "cassowary",
        "chasse",
        "chassis",
        "crass",
        "crevasse",
        "crevassing",
        "cuirass",
        "curassow",
        "dassie",
        "embarrass",
        "eyass",
        "frass",
        "fricassee",
        "galleass",
        "galliass",
        "gassed",
        "gasser",
        "gasses",
        "gassiest",
        "gassiness",
        "gassing",
        "gassy",
        "grass",
        "harass",
        "hassel",
        "hassium",
        "hassle",
        "hassling",
        "hassock",
        "jassid",
        "kavass",
        "kvass",
        "lass",
        "madrassa",
        "mass",
        "megass",
        "morass",
        "palliasse",
        "pass",
        "piassava",
        "quass",
        "rassle",
        "sargasso",
        "sargassum",
        "sass",
        "strass",
        "tass",
        "trass",
        "vassal",
        "vinasse",
        "wassail",
        "wrasse",
        "wrassle",
        // cock
        "bibcock",
        "cockade",
        "cockamamie",
        "cockamamy",
        "cockapoo",
        "cockateel",
        "cockatiel",
        "cockatrice",
        "cockatoo",
        "cockboat",
        "cockchafer",
        "cockcrow",
        "cocked",
        "cocker",
        "cockeye",
        "cockhorse",
        "cockier",
        "cockiest",
        "cockily",
        "cockiness",
        "cocking",
        "cockle",
        "cockling",
        "cockloft",
        "cockney",
        "cockpit",
        "cockroach",
        "cockscomb",
        "cockshut",
        "cockspur",
        "cocktail",
        "cocky",
        "gamecock",
        "gorcock",
        "haycock",
        "moorcock",
        "peacock",
        "petcock",
        "pinchcock",
        "poppycock",
        "recock",
        "seacock",
        "shuttlecock",
        "stopcock",
        "uncock",
        "weathercock",
        "woodcock",
        // cum
        // https://www.thefreedictionary.com/words-containing-cum 7-letter
        "acumen",
        "cumber",
        "cumbia",
        "cumin",
        "cumlaude",  // suma cum laude, etc.
        "cummin",
        "cumuli",
        "locum",
        "scum",
        "talcum",
        
    ],
    // Separators.
    &[" ", "_", "-", ".", "\n", "\t"],
    // Aliases.
    &[
        // Capitals.
        ("a", "A"),
        ("b", "B"),
        ("c", "C"),
        ("d", "D"),
        ("e", "E"),
        ("f", "F"),
        ("g", "G"),
        ("h", "H"),
        ("i", "I"),
        ("j", "J"),
        ("k", "K"),
        ("l", "L"),
        ("m", "M"),
        ("n", "N"),
        ("o", "O"),
        ("p", "P"),
        ("q", "Q"),
        ("r", "R"),
        ("s", "S"),
        ("t", "T"),
        ("u", "U"),
        ("v", "V"),
        ("w", "W"),
        ("x", "X"),
        ("y", "Y"),
        ("z", "Z"),
        // Look-alikes.
        ("a", "@"),
        ("a", "à"),
        ("a", "á"),
        ("a", "â"),
        ("a", "ã"),
        ("a", "ä"),
        ("a", "å"),
        ("a", "α"),
        ("A", "4"),
        ("A", "À"),
        ("A", "Á"),
        ("A", "Â"),
        ("A", "Ã"),
        ("A", "Ä"),
        ("A", "Å"),
        ("A", "Α"),
        ("A", "^"),
        ("A", "Д"),
        ("B", "ß"),
        ("B", "Β"),
        ("B", "8"),
        ("B", "฿"),
        ("c", "¢"),
        ("c", "ç"),
        ("c", "©"),
        ("c", "("),
        ("c", "["),
        ("c", "{"),
        ("c", "<"),
        ("C", "Ç"),
        ("d", "₫"),
        ("D", "Ð"),
        ("e", "è"),
        ("e", "é"),
        ("e", "ê"),
        ("e", "ë"),
        ("E", "3"),
        ("E", "£"),
        ("E", "€"),
        ("E", "È"),
        ("E", "É"),
        ("E", "Ê"),
        ("E", "Ë"),
        ("E", "ε"),
        ("E", "Ε"),
        ("E", "Ξ"),
        ("E", "Σ"),
        ("G", "6"),
        ("H", "Η"),
        ("H", "#"),
        ("i", "ì"),
        ("i", "í"),
        ("i", "î"),
        ("i", "ï"),
        ("I", "!"),
        ("I", "Ì"),
        ("I", "Í"),
        ("I", "Î"),
        ("I", "Ï"),
        ("I", "Ι"),
        ("I", "1"),
        ("I", "|"),
        ("k", "κ"),
        ("K", "Κ"),
        ("l", "1"),
        ("l", "|"),
        ("M", "Μ"),
        ("n", "ñ"),
        ("n", "η"),
        ("n", "Π"),
        ("N", "Ñ"),
        ("N", "Ν"),
        ("o", "ò"),
        ("o", "ó"),
        ("o", "ô"),
        ("o", "õ"),
        ("o", "ö"),
        ("o", "ø"),
        ("o", "ο"),
        ("o", "σ"),
        ("O", "0"),
        ("O", "Ò"),
        ("O", "Ó"),
        ("O", "Ô"),
        ("O", "Õ"),
        ("O", "Ö"),
        ("O", "Ø"),
        ("O", "θ"),
        ("O", "Θ"),
        ("O", "Ο"),
        ("O", "Φ"),
        ("p", "ρ"),
        ("p", "℗"),
        ("p", "þ"),
        ("P", "Ρ"),
        ("P", "₱"),
        ("P", "Þ"),
        ("R", "®"),
        ("R", "Я"),
        ("S", "5"),
        ("S", "$"),
        ("t", "+"),
        ("T", "τ"),
        ("T", "T"),
        ("T", "7"),
        ("u", "ù"),
        ("u", "ú"),
        ("u", "û"),
        ("u", "ü"),
        ("u", "μ"),
        ("u", "υ"),
        ("U", "Ù"),
        ("U", "Ú"),
        ("U", "Û"),
        ("U", "Ü"),
        ("v", "ν"),
        ("w", "ω"),
        ("W", "₩"),
        ("x", "×"),
        ("X", "χ"),
        ("X", "X"),
        ("y", "ý"),
        ("y", "ÿ"),
        ("y", "γ"),
        ("Y", "¥"),
        ("Y", "Ý"),
        ("Y", "Υ"),
        ("Z", "2"),
        ("Z", "Ζ"),
        // Slang.
        ("ck", "k"),
        ("er", "a"),
        ("ing", "in"),
        ("ing", "in'"),
        // Multi-character expansions.
        ("a", "(L"),
        ("A", "/\\"),
        ("A", "/-\\"),
        ("B", "I3"),
        ("M", "|\\/|"),
        ("N", "|\\|"),
        ("U", "|_|"),
        // Emoji.
        ("🖕", "🖕🏻"),
        ("🖕", "🖕🏼"),
        ("🖕", "🖕🏽"),
        ("🖕", "🖕🏾"),
        ("🖕", "🖕🏿"),
    ],
    Options::default(),
));

#[cfg(test)]
mod tests {
    use crate::PROFANITY_FILTER;

    #[test]
    fn finds_bad_words() {
        assert_eq!(PROFANITY_FILTER.censor("FuCk"), "****");
        assert_eq!(PROFANITY_FILTER.censor("nigga"), "*****");
        assert_eq!(PROFANITY_FILTER.censor("🖕🏿"), "**");
        assert_eq!(PROFANITY_FILTER.censor("@$$"), "***");
    }

    #[test]
    fn long() {
        //dbg!(PROFANITY_FILTER.find("What the fuck did you just fucking say about me, you little bitch? I’ll have you know I graduated top of my class in the Navy Seals, and I’ve been involved in numerous secret raids on Al-Quaeda, and I have over 300 confirmed kills. I am trained in gorilla warfare and I’m the top sniper in the entire US armed forces. You are nothing to me but just another target. I will wipe you the fuck out with precision the likes of which has never been seen before on this Earth, mark my fucking words. You think you can get away with saying that shit to me over the Internet? Think again, fucker. As we speak I am contacting my secret network of spies across the USA and your IP is being traced right now so you better prepare for the storm, maggot. The storm that wipes out the pathetic little thing you call your life. You’re fucking dead, kid. I can be anywhere, anytime, and I can kill you in over seven hundred ways, and that’s just with my bare hands. Not only am I extensively trained in unarmed combat, but I have access to the entire arsenal of the United States Marine Corps and I will use it to its full extent to wipe your miserable ass off the face of the continent, you little shit. If only you could have known what unholy retribution your little “clever” comment was about to bring down upon you, maybe you would have held your fucking tongue. But you couldn’t, you didn’t, and now you’re paying the price, you goddamn idiot. I will shit fury all over you and you will drown in it. You’re fucking dead, kiddo."));
        assert!(!PROFANITY_FILTER.check("as someone once said,"));
    }
}
