use once_cell::sync::Lazy;
use word_filter::{WordFilter, WordFilterBuilder};

pub static PROFANITY_FILTER: Lazy<WordFilter<'static>> = Lazy::new(|| {
    // Probably the best source on the web for words:
    // https://www.reddit.com/r/copypasta/comments/fca22g/every_swear_word_in_alphabetical_order/

    let mut builder = WordFilterBuilder::new();
    builder
        .separators(&[" ", "_", "-", ".", "\n", "\t"])
        // Diacritical marks.
        .separators('\u{300}'..='\u{36f}')
        // Cryllic marks.
        .separators('\u{483}'..='\u{487}')
        // Hebrew marks.
        .separators('\u{591}'..='\u{5bd}')
        .separator(&'\u{5bf}')
        .separators('\u{5c1}'..='\u{5c2}')
        .separators('\u{5c4}'..='\u{5c5}')
        .separator(&'\u{5c7}')
        // Arabic marks.
        .separators('\u{610}'..='\u{61a}')
        .separators('\u{64b}'..='\u{65f}')
        .separator(&'\u{670}')
        .separators('\u{6d6}'..='\u{6ed}')
        .separators('\u{8d3}'..='\u{8ff}')
        // Syriac marks.
        .separator(&'\u{711}')
        .separators('\u{730}'..='\u{74a}')
        // Thaana marks.
        .separators('\u{7a6}'..='\u{7b0}')
        // Nko marks.
        .separators('\u{7eb}'..='\u{7f3}')
        .separator(&'\u{7fd}')
        // Samaritan marks.
        .separators('\u{816}'..='\u{823}')
        .separators('\u{825}'..='\u{827}')
        .separators('\u{829}'..='\u{82d}')
        // Mandaic marks.
        .separators('\u{859}'..='\u{85b}')
        // Devanagari marks.
        .separators('\u{900}'..='\u{902}')
        .separator(&'\u{93a}')
        .separator(&'\u{93c}')
        .separators('\u{941}'..='\u{948}')
        .separator(&'\u{94d}')
        .separators('\u{951}'..='\u{957}')
        .separators('\u{962}'..='\u{963}')
        // Bengali marks.
        .separator(&'\u{981}')
        .separator(&'\u{9bc}')
        .separators('\u{9c1}'..='\u{9c4}')
        .separator(&'\u{9cd}')
        .separators('\u{9e2}'..='\u{9e3}')
        .separator(&'\u{9fe}')
        // Gurmukhi marks.
        .separators('\u{a01}'..='\u{a02}')
        .separator(&'\u{a3c}')
        .separators('\u{a41}'..='\u{a42}')
        .separators('\u{a47}'..='\u{a48}')
        .separators('\u{a4b}'..='\u{a4d}')
        .separator(&'\u{a51}')
        .separators('\u{a70}'..='\u{a71}')
        .separator(&'\u{a75}')
        // Gujarati marks.
        .separators('\u{a81}'..='\u{a82}')
        .separator(&'\u{abc}')
        .separators('\u{ac1}'..='\u{ac5}')
        .separators('\u{ac7}'..='\u{ac8}')
        .separator(&'\u{acd}')
        .separators('\u{ae2}'..='\u{ae3}')
        .separators('\u{afa}'..='\u{aff}')
        // Oriya marks.
        .separator(&'\u{b01}')
        .separator(&'\u{b3c}')
        .separator(&'\u{b3f}')
        .separators('\u{b41}'..='\u{b44}')
        .separator(&'\u{b4d}')
        .separator(&'\u{b56}')
        .separators('\u{b62}'..='\u{b63}')
        // Tamil marks.
        .separator(&'\u{b82}')
        .separator(&'\u{bc0}')
        .separator(&'\u{bcd}')
        // Telugu marks.
        .separator(&'\u{c00}')
        .separator(&'\u{c04}')
        .separators('\u{c3e}'..='\u{c3f}')
        .separator(&'\u{c40}')
        .separators('\u{c46}'..='\u{c48}')
        .separators('\u{c4a}'..='\u{c4d}')
        .separators('\u{c55}'..='\u{c56}')
        .separators('\u{c62}'..='\u{c63}')
        // Kannada marks.
        .separator(&'\u{c81}')
        .separator(&'\u{cbc}')
        .separator(&'\u{cbf}')
        .separator(&'\u{cc6}')
        .separators('\u{ccc}'..='\u{ccd}')
        .separators('\u{ce2}'..='\u{ce3}')
        // Malayalam marks.
        .separators('\u{d00}'..='\u{d01}')
        .separators('\u{d3b}'..='\u{d3c}')
        .separators('\u{d41}'..='\u{d44}')
        .separator(&'\u{d4d}')
        .separators('\u{d62}'..='\u{d63}')
        // Sinhala marks.
        .separator(&'\u{dca}')
        .separators('\u{dd2}'..='\u{dd4}')
        .separator(&'\u{dd6}')
        .aliases(&[
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
            ("th", "d"),
            // Multi-character expansions.
            ("a", "(L"),
            ("A", "/\\"),
            ("A", "/-\\"),
            ("B", "I3"),
            ("M", "|\\/|"),
            ("N", "|\\|"),
            ("U", "|_|"),
            // Squared Letters.
            ("a", "🅰"),
            ("b", "🅱"),
            ("c", "🅲"),
            ("d", "🅳"),
            ("d", "🆥"),
            ("e", "🅴"),
            ("f", "🅵"),
            ("g", "🅶"),
            ("h", "🅷"),
            ("i", "🅸"),
            ("j", "🅹"),
            ("k", "🅺"),
            ("l", "🅻"),
            ("m", "🅼"),
            ("n", "🅽"),
            ("o", "🅾"),
            ("p", "🅿"),
            ("p", "🆊"),
            ("q", "🆀"),
            ("r", "🆁"),
            ("s", "🆂"),
            ("t", "🆃"),
            ("u", "🆄"),
            ("v", "🆅"),
            ("w", "🆆"),
            ("x", "🆇"),
            ("y", "🆈"),
            ("z", "🆉"),
            ("ic", "🆋"),
            ("pa", "🆌"),
            ("sa", "🆍"),
            ("ab", "🆎"),
            ("wc", "🆏"),
            ("dj", "🆐"),
            ("cl", "🆑"),
            ("id", "🆔"),
            ("ng", "🆖"),
            // Large Letters
            ("A", "🇦"),
            ("B", "🇧"),
            ("C", "🇨"),
            ("D", "🇩"),
            ("E", "🇪"),
            ("F", "🇫"),
            ("G", "🇬"),
            ("H", "🇭"),
            ("I", "🇮"),
            ("J", "🇯"),
            ("K", "🇰"),
            ("L", "🇱"),
            ("M", "🇲"),
            ("N", "🇳"),
            ("O", "🇴"),
            ("P", "🇵"),
            ("Q", "🇶"),
            ("R", "🇷"),
            ("S", "🇸"),
            ("T", "🇹"),
            ("U", "🇺"),
            ("V", "🇻"),
            ("W", "🇼"),
            ("X", "🇽"),
            ("Y", "🇾"),
            ("Z", "🇿"),
            // Emoji.
            ("a", "🅰️"),
            ("ab", "🆎"),
            ("b", "🅱️"),
            ("e", "📧"),
            ("h", "♓"),
            ("i", "ℹ️"),
            ("id", "🆔"),
            ("m", "Ⓜ"),
            ("o", "🅾️"),
            ("p", "🅿️"),
            ("u", "⛎"),
            ("v", "♈"),
            ("z", "Ⓩ"),
        ]);

    // Profane.
    if cfg!(feature = "damn") {
        builder
            .words(&["damn", "damnit", "goddamn", "goddamnit"])
            .aliases(&[("damnit", "dammit")]);
    }

    if cfg!(feature = "hell") {
        builder.words(&["hell", "hella", "hells"]).exceptions(&[
            "echelle",
            "hatchelled",
            "hatchelling",
            "hellebore",
            "helleri",
            "hellgrammite",
            "hello",
            "phellem",
            "phelloderm",
            "phellogen",
            "philhellene",
            "philhellenic",
            "shell",
            "zooxanthella",
        ]);
    }

    // Vulgar.
    if cfg!(feature = "anal") {
        builder
            .words(&["anal", "analplug", "analplugs"])
            .exceptions(&[
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
            ]);
    }

    if cfg!(feature = "ass") {
        builder
            .words(&[
                "ass",
                "asses",
                "asshole",
                "dumbass",
                "dumbasses",
                "jackass",
                "jackasses",
                "jackassery",
            ])
            .exceptions(&[
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
            ]);
    }

    if cfg!(feature = "blow-job") {
        builder.words(&["blowjob", "blowjobs"]);
    }

    if cfg!(feature = "cunt") {
        builder.words(&["cunt", "cunts"]);
    }

    if cfg!(feature = "fuck") {
        builder
            .words(&[
                "fuck",
                "fucked",
                "fucker",
                "fuckers",
                "fucking",
                "fucks",
                "motherfuck",
                "motherfucker",
                "motherfucking",
            ])
            .aliases(&[("mother", "muther")]);
    }

    if cfg!(feature = "shit") {
        builder.words(&[
            "bullshit",
            "bullshits",
            "bullshitted",
            "bullshitting",
            "shat",
            "shit",
            "shits",
            "shitter",
            "shittier",
            "shittiest",
            "shitting",
            "shitty",
        ]);
    }

    // Offensive.
    if cfg!(feature = "bastard") {
        builder
            .words(&[
                "bastard",
                "bastardies",
                "bastardization",
                "bastardizations",
                "bastardize",
                "bastardized",
                "bastardizes",
                "bastardizing",
                "bastardly",
                "bastards",
                "bastardy",
            ])
            .aliases(&[
                ("bastardize", "bastardise"),
                ("bastardizing", "bastardising"),
            ]);
    }

    if cfg!(feature = "bitch") {
        builder.words(&[
            "bitch",
            "bitched",
            "bitcheries",
            "bitchery",
            "bitches",
            "bitchier",
            "bitchiest",
            "bitchily",
            "bitchiness",
            "bitchinesses",
            "bitching",
            "bitchy",
        ]);
    }

    if cfg!(feature = "faggot") {
        builder.words(&[
            "fag",
            "fagged",
            "faggier",
            "fagging",
            "faggot",
            "faggoted",
            "faggotries",
            "faggotry",
            "faggots",
            "faggoty",
            "faggy",
            "fags",
        ]);
    }

    if cfg!(feature = "nigger") {
        builder
            .words(&["nigger", "niggers"])
            .aliases(&[("nigger", "nibber")]);
    }

    if cfg!(feature = "slut") {
        builder.words(&[
            "slut",
            "sluts",
            "slutted",
            "sluttier",
            "sluttiest",
            "slutting",
            "sluttish",
            "sluttishly",
            "slutty",
        ]);
    }

    // Emoji.
    if cfg!(feature = "middle-finger-emoji") {
        builder.words(&["🖕"]).aliases(&[
            ("🖕", "🖕🏻"),
            ("🖕", "🖕🏼"),
            ("🖕", "🖕🏽"),
            ("🖕", "🖕🏾"),
            ("🖕", "🖕🏿"),
        ]);
    }

    builder.build()
});

// pub static PROFANITY_FILTER: Lazy<WordFilter<'static>> = Lazy::new(|| WordFilter::new(
//     // Filtered words.
//     &[
//         // Probably the best source on the web for these words:
//         // https://www.reddit.com/r/copypasta/comments/fca22g/every_swear_word_in_alphabetical_order/

//         // Vulgar.
//         "cock",
//         "cocks",
//         "cocksucker",
//         "cocksuckers",
//         "cocksucking",
//         "cum",
//         "cummed",
//         "cumming",
//         "cums",
//         "cumshot",
//         "cunt",
//         "dick",
//         "dickhead",
//         "jerkoff",
//         "jerkedoff",
//         "jerkingoff",
//         "jizz",
//         "jizzed",
//         "jizzing",
//         "pussy",
//         "sex",
//         // Offensive.
//         "chink",
//         "coon",
//     ],
//     // Exceptions.
//     &[
//         // cock
//         "bibcock",
//         "cockade",
//         "cockamamie",
//         "cockamamy",
//         "cockapoo",
//         "cockateel",
//         "cockatiel",
//         "cockatrice",
//         "cockatoo",
//         "cockboat",
//         "cockchafer",
//         "cockcrow",
//         "cocked",
//         "cocker",
//         "cockeye",
//         "cockhorse",
//         "cockier",
//         "cockiest",
//         "cockily",
//         "cockiness",
//         "cocking",
//         "cockle",
//         "cockling",
//         "cockloft",
//         "cockney",
//         "cockpit",
//         "cockroach",
//         "cockscomb",
//         "cockshut",
//         "cockspur",
//         "cocktail",
//         "cocky",
//         "gamecock",
//         "gorcock",
//         "haycock",
//         "moorcock",
//         "peacock",
//         "petcock",
//         "pinchcock",
//         "poppycock",
//         "recock",
//         "seacock",
//         "shuttlecock",
//         "stopcock",
//         "uncock",
//         "weathercock",
//         "woodcock",
//         // cum
//         // https://www.thefreedictionary.com/words-containing-cum 7-letter
//         "acumen",
//         "cumber",
//         "cumbia",
//         "cumin",
//         "cumlaude",  // suma cum laude, etc.
//         "cummin",
//         "cumuli",
//         "locum",
//         "scum",
//         "talcum",

#[cfg(test)]
mod tests {
    use crate::PROFANITY_FILTER;

    #[test]
    #[cfg_attr(not(feature = "damn"), ignore)]
    fn damn() {
        assert_eq!(
            PROFANITY_FILTER.find("Damn"),
            vec!["damn"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("dammit"),
            vec!["damnit"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("God damn!"),
            vec!["goddamn"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("god DAMMIT!"),
            vec!["goddamnit"].into_boxed_slice()
        );
    }

    #[test]
    #[cfg_attr(not(feature = "hell"), ignore)]
    fn hell() {
        assert_eq!(
            PROFANITY_FILTER.find("H3LL"),
            vec!["hell"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("hells yeah!"),
            vec!["hells"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("HELLA"),
            vec!["hella"].into_boxed_slice()
        );
        assert_eq!(PROFANITY_FILTER.find("hello"), vec![].into_boxed_slice());
    }

    #[test]
    #[cfg_attr(not(feature = "anal"), ignore)]
    fn anal() {
        assert_eq!(
            PROFANITY_FILTER.find("anAl"),
            vec!["anal"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("ANALPLUG"),
            vec!["analplug"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("anal plugs"),
            vec!["analplugs"].into_boxed_slice()
        );
        assert_eq!(PROFANITY_FILTER.find("analysis"), vec![].into_boxed_slice());
    }

    #[test]
    #[cfg_attr(not(feature = "ass"), ignore)]
    fn ass() {
        assert_eq!(PROFANITY_FILTER.find("ass"), vec!["ass"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("@$$"), vec!["ass"].into_boxed_slice());
        assert_eq!(
            PROFANITY_FILTER.find("asses"),
            vec!["asses"].into_boxed_slice()
        );
        assert_eq!(PROFANITY_FILTER.find("assess"), vec![].into_boxed_slice());
        assert_eq!(
            PROFANITY_FILTER.find("aSS hOLe"),
            vec!["asshole"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("JACKASS"),
            vec!["jackass"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("jackassery"),
            vec!["jackassery"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("jack-asses"),
            vec!["jackasses"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("as someone once said,"),
            vec![].into_boxed_slice()
        );
    }

    #[test]
    #[cfg_attr(not(feature = "blow-job"), ignore)]
    fn blow_job() {
        assert_eq!(PROFANITY_FILTER.find("blow job"), vec!["blowjob"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("🅱lowjobs"), vec!["blowjobs"].into_boxed_slice());
    }

    #[test]
    #[cfg_attr(not(feature = "cunt"), ignore)]
    fn cunt() {
        assert_eq!(PROFANITY_FILTER.find("CU|\\|T"), vec!["cunt"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("cunts"), vec!["cunts"].into_boxed_slice());
    }

    #[test]
    #[cfg_attr(not(feature = "fuck"), ignore)]
    fn fuck() {
        assert_eq!(
            PROFANITY_FILTER.find("fuck"),
            vec!["fuck"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("MUTHAFUKA"),
            vec!["motherfucker"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("motherfuckin'"),
            vec!["motherfucking"].into_boxed_slice()
        );
    }

    #[test]
    #[cfg_attr(not(feature = "shit"), ignore)]
    fn shit() {
        assert_eq!(
            PROFANITY_FILTER.find("shit"),
            vec!["shit"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("$H1TTY"),
            vec!["shitty"].into_boxed_slice()
        );
    }

    #[test]
    #[cfg_attr(not(feature = "bastard"), ignore)]
    fn bastard() {
        assert_eq!(
            PROFANITY_FILTER.find("bastard"),
            vec!["bastard"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("YOU BASTARDS!"),
            vec!["bastards"].into_boxed_slice()
        );
    }

    #[test]
    #[cfg_attr(not(feature = "bitch"), ignore)]
    fn bitch() {
        assert_eq!(
            PROFANITY_FILTER.find("that's bitchin'"),
            vec!["bitching"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("BitCh"),
            vec!["bitch"].into_boxed_slice()
        );
    }

    #[test]
    #[cfg_attr(not(feature = "faggot"), ignore)]
    fn faggot() {
        assert_eq!(PROFANITY_FILTER.find("FAG"), vec!["fag"].into_boxed_slice());
        assert_eq!(
            PROFANITY_FILTER.find("faggy"),
            vec!["faggy"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("FaGgOtS"),
            vec!["faggots"].into_boxed_slice()
        );
    }

    #[test]
    #[cfg_attr(not(feature = "nigger"), ignore)]
    fn nigger() {
        assert_eq!(
            PROFANITY_FILTER.find("nigger"),
            vec!["nigger"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("NI🅱️🅱️AS"),
            vec!["niggers"].into_boxed_slice()
        );
        assert_eq!(PROFANITY_FILTER.censor("NI🅱️🅱️AS"), "******");
    }

    #[test]
    #[cfg_attr(not(feature = "slut"), ignore)]
    fn slut() {
        assert_eq!(
            PROFANITY_FILTER.find("slut"),
            vec!["slut"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("5lut"),
            vec!["slut"].into_boxed_slice()
        );
        assert_eq!(
            PROFANITY_FILTER.find("SLUTTIEST"),
            vec!["sluttiest"].into_boxed_slice()
        );
    }

    #[test]
    #[cfg_attr(not(feature = "middle-finger-emoji"), ignore)]
    fn middle_finger_emoji() {
        assert_eq!(PROFANITY_FILTER.find("🖕"), vec!["🖕"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("🖕🏻"), vec!["🖕"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("🖕🏼"), vec!["🖕"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("🖕🏽"), vec!["🖕"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("🖕🏾"), vec!["🖕"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.find("🖕🏿"), vec!["🖕"].into_boxed_slice());
        assert_eq!(PROFANITY_FILTER.censor("🖕"), "*");
        assert_eq!(PROFANITY_FILTER.censor("🖕🏻"), "*");
        assert_eq!(PROFANITY_FILTER.censor("🖕🏼"), "*");
        assert_eq!(PROFANITY_FILTER.censor("🖕🏽"), "*");
        assert_eq!(PROFANITY_FILTER.censor("🖕🏾"), "*");
        assert_eq!(PROFANITY_FILTER.censor("🖕🏿"), "*");
    }
}
