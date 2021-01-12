use once_cell::sync::Lazy;
use word_filter::{Options, WordFilter};

pub static PROFANITY_FILTER: Lazy<WordFilter<'static>> = Lazy::new(|| WordFilter::new(
    // Filtered words.
    &[
        "anal",
        "analplug",
        "ass",
        "asshole",
        "bastard",
        "bitch",
        "blowjob",
        "bullshit",
        "chink",
        "cock",
        "cocksucker",
        "coon",
        "cum",
        "cumshot",
        "cunt",
        "damn",
        "dick",
        "dickhead",
        "fuck",
        "fucker",
        "motherfuck",
        "motherfucker",
        "nigger",
        "pussy",
        "sex",
        "shit",
    ],
    // Exceptions.
    &[],
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
        ("B", "ß"),
        ("B", "Β"),
        ("B", "฿"),
        ("c", "¢"),
        ("c", "ç"),
        ("c", "©"),
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
        ("k", "κ"),
        ("K", "Κ"),
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
        ("S", "5"),
        ("S", "$"),
        ("T", "τ"),
        ("T", "T"),
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
    }
}
