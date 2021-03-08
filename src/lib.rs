use once_cell::sync::Lazy;
use word_filter::{Options, WordFilter};

pub static PROFANITY_FILTER: Lazy<WordFilter<'static>> = Lazy::new(|| WordFilter::new(
    // Filtered words.
    &[
        // Probably the best source on the web for these words:
        // https://www.reddit.com/r/copypasta/comments/fca22g/every_swear_word_in_alphabetical_order/

        // Profane.
        "damn",
        "damnit",
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
        "blowjobs",
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
        "fag",
        "faggot",
        "faggots",
        "fags",
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
        "jizzed",
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
        ("damnit", "dammit"),
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
    fn damn() {
        assert_eq!(PROFANITY_FILTER.find("Damn"), vec!["damn"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("dammit"), vec!["damnit"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("God damn!"), vec!["goddamn"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("god DAMMIT!"), vec!["goddamnit"].into_boxed_slice());
    }

    #[test]
    fn anal() {
        assert_eq!(PROFANITY_FILTER.find("anAl"), vec!["anal"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("ANALPLUG"), vec!["analplug"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("anal plugs"), vec!["analplugs"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("analysis"), vec![].into_boxed_slice());
    }

    #[test]
    fn ass() {
        assert_eq!(PROFANITY_FILTER.find("ass"), vec!["ass"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("@$$"), vec!["ass"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("asses"), vec!["asses"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("assess"), vec![].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("aSS hOLe"), vec!["asshole"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("JACKASS"), vec!["jackass"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("jackassery"), vec!["jackassery"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("jack-asses"), vec!["jackasses"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("as someone once said,"), vec![].into_boxed_slice());
    }

    #[test]
    fn finds_bad_words() {
        assert_eq!(PROFANITY_FILTER.censor("FuCk"), "****");
        assert_eq!(PROFANITY_FILTER.censor("nigga"), "*****");
        assert_eq!(PROFANITY_FILTER.censor("🖕🏿"), "**");
        assert_eq!(PROFANITY_FILTER.censor("@$$"), "***");
    }
}
