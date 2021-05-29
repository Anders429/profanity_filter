#![no_std]

extern crate alloc;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

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
//         "jerkoff",
//         "jerkedoff",
//         "jerkingoff",
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
    use alloc::{vec, vec::Vec};

    #[test]
    #[cfg_attr(not(feature = "damn"), ignore)]
    fn damn() {
        assert_eq!(
            PROFANITY_FILTER.find("Damn").collect::<Vec<_>>(),
            vec!["damn"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("dammit").collect::<Vec<_>>(),
            vec!["damnit"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("God damn!").collect::<Vec<_>>(),
            vec!["goddamn"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("god DAMMIT!").collect::<Vec<_>>(),
            vec!["goddamnit"]
        );
    }

    #[test]
    #[cfg_attr(not(feature = "hell"), ignore)]
    fn hell() {
        assert_eq!(
            PROFANITY_FILTER.find("H3LL").collect::<Vec<_>>(),
            vec!["hell"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("hells yeah!").collect::<Vec<_>>(),
            vec!["hells"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("HELLA").collect::<Vec<_>>(),
            vec!["hella"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("hello").collect::<Vec<_>>(),
            Vec::<&str>::new()
        );
    }

    #[test]
    #[cfg_attr(not(feature = "anal"), ignore)]
    fn anal() {
        assert_eq!(
            PROFANITY_FILTER.find("anAl").collect::<Vec<_>>(),
            vec!["anal"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("ANALPLUG").collect::<Vec<_>>(),
            vec!["analplug"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("anal plugs").collect::<Vec<_>>(),
            vec!["analplugs"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("analysis").collect::<Vec<_>>(),
            Vec::<&str>::new()
        );
    }

    #[test]
    #[cfg_attr(not(feature = "ass"), ignore)]
    fn ass() {
        assert_eq!(
            PROFANITY_FILTER.find("ass").collect::<Vec<_>>(),
            vec!["ass"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("@$$").collect::<Vec<_>>(),
            vec!["ass"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("asses").collect::<Vec<_>>(),
            vec!["asses"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("assess").collect::<Vec<_>>(),
            Vec::<&str>::new()
        );
        assert_eq!(
            PROFANITY_FILTER.find("aSS hOLe").collect::<Vec<_>>(),
            vec!["asshole"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("JACKASS").collect::<Vec<_>>(),
            vec!["jackass"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("jackassery").collect::<Vec<_>>(),
            vec!["jackassery"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("jack-asses").collect::<Vec<_>>(),
            vec!["jackasses"]
        );
        assert_eq!(
            PROFANITY_FILTER
                .find("as someone once said,")
                .collect::<Vec<_>>(),
            Vec::<&str>::new(),
        );
    }

    #[test]
    #[cfg_attr(not(feature = "blow-job"), ignore)]
    fn blow_job() {
        assert_eq!(
            PROFANITY_FILTER.find("blow job").collect::<Vec<_>>(),
            vec!["blowjob"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("🅱lowjobs").collect::<Vec<_>>(),
            vec!["blowjobs"]
        );
    }

    #[test]
    #[cfg_attr(not(feature = "cunt"), ignore)]
    fn cunt() {
        assert_eq!(
            PROFANITY_FILTER.find("CU|\\|T").collect::<Vec<_>>(),
            vec!["cunt"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("cunts").collect::<Vec<_>>(),
            vec!["cunts"]
        );
    }

    #[test]
    #[cfg_attr(not(feature = "dick"), ignore)]
    fn dick() {
        assert_eq!(
            PROFANITY_FILTER.find("dick").collect::<Vec<_>>(),
            vec!["dick"]
        );
        assert_eq!(
            PROFANITY_FILTER
                .find("what the dickens!")
                .collect::<Vec<_>>(),
            Vec::<&str>::new()
        );
        assert_eq!(
            PROFANITY_FILTER.find("🅳ℹ️¢🇰 heads").collect::<Vec<_>>(),
            vec!["dickheads"]
        );
        assert_eq!(PROFANITY_FILTER.censor("🅳ℹ️¢🇰 heads"), "**********");
    }

    #[test]
    #[cfg_attr(not(feature = "fuck"), ignore)]
    fn fuck() {
        assert_eq!(
            PROFANITY_FILTER.find("fuck").collect::<Vec<_>>(),
            vec!["fuck"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("MUTHAFUKA").collect::<Vec<_>>(),
            vec!["motherfucker"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("motherfuckin'").collect::<Vec<_>>(),
            vec!["motherfucking"]
        );
        assert_eq!(PROFANITY_FILTER.censor("f̴̨̠͓͓͈̩̱̯͇̘̘̺͆̓͊u̴̧̝̲̘̭̥̰̣͕̼̞̝̝̥̦͉̺̾͑̅̂͊́̓̅̓̅͑͒̐̈́͆̑͠ͅç̶͖̞̠̯̦͕͇͍̟̝̪̮̲̗̈́̔̓̈́̈͆̃͋ͅk"), "****");
    }

    #[test]
    #[cfg_attr(not(feature = "jerk-off"), ignore)]
    fn jerk_off() {
        assert_eq!(PROFANITY_FILTER.find("jerking off").collect::<Vec<_>>(), vec!["jerkingoff"]);
        assert_eq!(PROFANITY_FILTER.find("jerk-0ff").collect::<Vec<_>>(), vec!["jerkoff"]);
    }

    #[test]
    #[cfg_attr(not(feature = "jizz"), ignore)]
    fn jizz() {
        assert_eq!(PROFANITY_FILTER.find("JIZZ").collect::<Vec<_>>(), vec!["jizz"]);
        assert_eq!(PROFANITY_FILTER.find("gizzums").collect::<Vec<_>>(), vec!["jizzes"]);
    }

    #[test]
    #[cfg_attr(not(feature = "penis"), ignore)]
    fn penis() {
        assert_eq!(
            PROFANITY_FILTER.find("p3n1s3s").collect::<Vec<_>>(),
            vec!["penises"]
        );
        assert_eq!(PROFANITY_FILTER.censor("PENIS"), "*****");
    }

    #[test]
    #[cfg_attr(not(feature = "shit"), ignore)]
    fn shit() {
        assert_eq!(
            PROFANITY_FILTER.find("shit").collect::<Vec<_>>(),
            vec!["shit"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("$H1TTY").collect::<Vec<_>>(),
            vec!["shitty"]
        );
    }

    #[test]
    #[cfg_attr(not(feature = "vagina"), ignore)]
    fn vagina() {
        assert_eq!(
            PROFANITY_FILTER.find("vagina").collect::<Vec<_>>(),
            vec!["vagina"]
        );
    }

    #[test]
    #[cfg_attr(not(feature = "bastard"), ignore)]
    fn bastard() {
        assert_eq!(
            PROFANITY_FILTER.find("bastard").collect::<Vec<_>>(),
            vec!["bastard"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("YOU BASTARDS!").collect::<Vec<_>>(),
            vec!["bastards"]
        );
    }

    #[test]
    #[cfg_attr(not(feature = "bitch"), ignore)]
    fn bitch() {
        assert_eq!(
            PROFANITY_FILTER.find("that's bitchin'").collect::<Vec<_>>(),
            vec!["bitching"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("BitCh").collect::<Vec<_>>(),
            vec!["bitch"]
        );
    }

    #[test]
    #[cfg_attr(not(feature = "faggot"), ignore)]
    fn faggot() {
        assert_eq!(
            PROFANITY_FILTER.find("FAG").collect::<Vec<_>>(),
            vec!["fag"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("faggy").collect::<Vec<_>>(),
            vec!["faggy"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("FaGgOtS").collect::<Vec<_>>(),
            vec!["faggots"]
        );
    }

    #[test]
    #[cfg_attr(not(feature = "nigger"), ignore)]
    fn nigger() {
        assert_eq!(
            PROFANITY_FILTER.find("nigger").collect::<Vec<_>>(),
            vec!["nigger"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("NI🅱️🅱️AS").collect::<Vec<_>>(),
            vec!["niggers"]
        );
        assert_eq!(PROFANITY_FILTER.censor("NI🅱️🅱️AS"), "******");
    }

    #[test]
    #[cfg_attr(not(feature = "slut"), ignore)]
    fn slut() {
        assert_eq!(
            PROFANITY_FILTER.find("slut").collect::<Vec<_>>(),
            vec!["slut"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("5lut").collect::<Vec<_>>(),
            vec!["slut"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("SLUTTIEST").collect::<Vec<_>>(),
            vec!["sluttiest"]
        );
    }

    #[test]
    #[cfg_attr(not(feature = "middle-finger-emoji"), ignore)]
    fn middle_finger_emoji() {
        assert_eq!(PROFANITY_FILTER.find("🖕").collect::<Vec<_>>(), vec!["🖕"]);
        assert_eq!(
            PROFANITY_FILTER.find("🖕🏻").collect::<Vec<_>>(),
            vec!["🖕"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("🖕🏼").collect::<Vec<_>>(),
            vec!["🖕"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("🖕🏽").collect::<Vec<_>>(),
            vec!["🖕"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("🖕🏾").collect::<Vec<_>>(),
            vec!["🖕"]
        );
        assert_eq!(
            PROFANITY_FILTER.find("🖕🏿").collect::<Vec<_>>(),
            vec!["🖕"]
        );
        assert_eq!(PROFANITY_FILTER.censor("🖕"), "*");
        assert_eq!(PROFANITY_FILTER.censor("🖕🏻"), "*");
        assert_eq!(PROFANITY_FILTER.censor("🖕🏼"), "*");
        assert_eq!(PROFANITY_FILTER.censor("🖕🏽"), "*");
        assert_eq!(PROFANITY_FILTER.censor("🖕🏾"), "*");
        assert_eq!(PROFANITY_FILTER.censor("🖕🏿"), "*");
    }
}
