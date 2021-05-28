use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};
use word_filter_codegen::{Visibility, WordFilterGenerator};

fn main() {
    let file = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&file).unwrap());

    let mut generator = WordFilterGenerator::new();
    generator
        .visibility(Visibility::Pub)
        .separators(&[" ", "_", "-", ".", "\n", "\t"])
        // Non-spacing marks.
        .separators('\u{300}'..='\u{36f}')
        .separators('\u{483}'..='\u{487}')
        .separators('\u{591}'..='\u{5bd}')
        .separator('\u{5bf}')
        .separators('\u{5c1}'..='\u{5c2}')
        .separators('\u{5c4}'..='\u{5c5}')
        .separator('\u{5c7}')
        .separators('\u{610}'..='\u{61a}')
        .separators('\u{64b}'..='\u{65f}')
        .separator('\u{670}')
        .separators('\u{6d6}'..='\u{6dc}')
        .separators('\u{6df}'..='\u{6e4}')
        .separators('\u{6e7}'..='\u{6e8}')
        .separators('\u{6ea}'..='\u{6ed}')
        .separator('\u{711}')
        .separators('\u{730}'..='\u{74a}')
        .separators('\u{7a6}'..='\u{7b0}')
        .separators('\u{7eb}'..='\u{7f3}')
        .separator('\u{7fd}')
        .separators('\u{816}'..='\u{819}')
        .separators('\u{81b}'..='\u{823}')
        .separators('\u{825}'..='\u{827}')
        .separators('\u{829}'..='\u{82d}')
        .separators('\u{859}'..='\u{85b}')
        .separators('\u{8d3}'..='\u{8e1}')
        .separators('\u{8e3}'..='\u{902}')
        .separator('\u{93a}')
        .separator('\u{93c}')
        .separators('\u{941}'..='\u{948}')
        .separator('\u{94d}')
        .separators('\u{951}'..='\u{957}')
        .separators('\u{962}'..='\u{963}')
        .separator('\u{981}')
        .separator('\u{9bc}')
        .separators('\u{9c1}'..='\u{9c4}')
        .separator('\u{9cd}')
        .separators('\u{9e2}'..='\u{9e3}')
        .separator('\u{9fe}')
        .separators('\u{a01}'..='\u{a02}')
        .separator('\u{a3c}')
        .separators('\u{a41}'..='\u{a42}')
        .separators('\u{a47}'..='\u{a48}')
        .separators('\u{a4b}'..='\u{a4d}')
        .separator('\u{a51}')
        .separators('\u{a70}'..='\u{a71}')
        .separator('\u{a75}')
        .separators('\u{a81}'..='\u{a82}')
        .separator('\u{abc}')
        .separators('\u{ac1}'..='\u{ac5}')
        .separators('\u{ac7}'..='\u{ac8}')
        .separator('\u{acd}')
        .separators('\u{ae2}'..='\u{ae3}')
        .separators('\u{afa}'..='\u{aff}')
        .separator('\u{b01}')
        .separator('\u{b3c}')
        .separator('\u{b3f}')
        .separators('\u{b41}'..='\u{b44}')
        .separator('\u{b4d}')
        .separators('\u{b55}'..='\u{b56}')
        .separators('\u{b62}'..='\u{b63}')
        .separator('\u{b82}')
        .separator('\u{bc0}')
        .separator('\u{bcd}')
        .separator('\u{c00}')
        .separator('\u{c04}')
        .separators('\u{c3e}'..='\u{c40}')
        .separators('\u{c46}'..='\u{c48}')
        .separators('\u{c4a}'..='\u{c4d}')
        .separators('\u{c55}'..='\u{c56}')
        .separators('\u{c62}'..='\u{c63}')
        .separator('\u{c81}')
        .separator('\u{cbc}')
        .separator('\u{cbf}')
        .separator('\u{cc6}')
        .separators('\u{ccc}'..='\u{ccd}')
        .separators('\u{ce2}'..='\u{ce3}')
        .separators('\u{d00}'..='\u{d01}')
        .separators('\u{d3b}'..='\u{d3c}')
        .separators('\u{d41}'..='\u{d44}')
        .separator('\u{d4d}')
        .separators('\u{d62}'..='\u{d63}')
        .separator('\u{d81}')
        .separator('\u{dca}')
        .separators('\u{dd2}'..='\u{dd4}')
        .separator('\u{dd6}')
        .separator('\u{e31}')
        .separators('\u{e34}'..='\u{e3a}')
        .separators('\u{e47}'..='\u{e4e}')
        .separator('\u{eb1}')
        .separators('\u{eb4}'..='\u{ebc}')
        .separators('\u{ec8}'..='\u{ecd}')
        .separators('\u{f18}'..='\u{f19}')
        .separator('\u{f35}')
        .separator('\u{f37}')
        .separator('\u{f39}')
        .separators('\u{f71}'..='\u{f7e}')
        .separators('\u{f80}'..='\u{f84}')
        .separators('\u{f86}'..='\u{f87}')
        .separators('\u{f8d}'..='\u{f97}')
        .separators('\u{f99}'..='\u{fbc}')
        .separator('\u{fc6}')
        .separators('\u{102d}'..='\u{1030}')
        .separators('\u{1032}'..='\u{1037}')
        .separators('\u{1039}'..='\u{103a}')
        .separators('\u{103d}'..='\u{103e}')
        .separators('\u{1058}'..='\u{1059}')
        .separators('\u{105e}'..='\u{1060}')
        .separators('\u{1071}'..='\u{1074}')
        .separator('\u{1082}')
        .separators('\u{1085}'..='\u{1086}')
        .separator('\u{108d}')
        .separator('\u{109d}')
        .separators('\u{135d}'..='\u{135f}')
        .separators('\u{1712}'..='\u{1714}')
        .separators('\u{1732}'..='\u{1734}')
        .separators('\u{1752}'..='\u{1753}')
        .separators('\u{1772}'..='\u{1773}')
        .separators('\u{17b4}'..='\u{17b5}')
        .separators('\u{17b7}'..='\u{17bd}')
        .separator('\u{17c6}')
        .separators('\u{17c9}'..='\u{17d3}')
        .separator('\u{17dd}')
        .separators('\u{180b}'..='\u{180d}')
        .separators('\u{1885}'..='\u{1886}')
        .separator('\u{18a9}')
        .separators('\u{1920}'..='\u{1922}')
        .separators('\u{1927}'..='\u{1928}')
        .separator('\u{1932}')
        .separators('\u{1939}'..='\u{193b}')
        .separators('\u{1a17}'..='\u{1a18}')
        .separator('\u{1a1b}')
        .separator('\u{1a56}')
        .separators('\u{1a58}'..='\u{1a5e}')
        .separator('\u{1a60}')
        .separator('\u{1a62}')
        .separators('\u{1a65}'..='\u{1a6c}')
        .separators('\u{1a73}'..='\u{1a7c}')
        .separator('\u{1a7f}')
        .separators('\u{1ab0}'..='\u{1abd}')
        .separators('\u{1abf}'..='\u{1ac0}')
        .separators('\u{1b00}'..='\u{1b03}')
        .separator('\u{1b34}')
        .separators('\u{1b36}'..='\u{1b3a}')
        .separator('\u{1b3c}')
        .separator('\u{1b42}')
        .separators('\u{1b6b}'..='\u{1b73}')
        .separators('\u{1b80}'..='\u{1b81}')
        .separators('\u{1ba2}'..='\u{1ba5}')
        .separators('\u{1ba8}'..='\u{1ba9}')
        .separators('\u{1bab}'..='\u{1bad}')
        .separator('\u{1be6}')
        .separators('\u{1be8}'..='\u{1be9}')
        .separator('\u{1bed}')
        .separators('\u{1bef}'..='\u{1bf1}')
        .separators('\u{1c2c}'..='\u{1c33}')
        .separators('\u{1c36}'..='\u{1c37}')
        .separators('\u{1cd0}'..='\u{1cd2}')
        .separators('\u{1cd4}'..='\u{1ce0}')
        .separators('\u{1ce2}'..='\u{1ce8}')
        .separator('\u{1ced}')
        .separator('\u{1cf4}')
        .separators('\u{1cf8}'..='\u{1cf9}')
        .separators('\u{1dc0}'..='\u{1df9}')
        .separators('\u{1dfb}'..='\u{1dff}')
        .separators('\u{20d0}'..='\u{20dc}')
        .separator('\u{20e1}')
        .separators('\u{20e5}'..='\u{20f0}')
        .separators('\u{2cef}'..='\u{2cf1}')
        .separator('\u{2d7f}')
        .separators('\u{2de0}'..='\u{2dff}')
        .separators('\u{302a}'..='\u{302d}')
        .separators('\u{3099}'..='\u{309a}')
        .separator('\u{a66f}')
        .separators('\u{a674}'..='\u{a67d}')
        .separators('\u{a69e}'..='\u{a69f}')
        .separators('\u{a6f0}'..='\u{a6f1}')
        .separator('\u{a802}')
        .separator('\u{a806}')
        .separator('\u{a80b}')
        .separators('\u{a825}'..='\u{a826}')
        .separator('\u{a82c}')
        .separators('\u{a8c4}'..='\u{a8c5}')
        .separators('\u{a8e0}'..='\u{a8f1}')
        .separator('\u{a8ff}')
        .separators('\u{a926}'..='\u{a92d}')
        .separators('\u{a947}'..='\u{a951}')
        .separators('\u{a980}'..='\u{a982}')
        .separator('\u{a9b3}')
        .separators('\u{a9b6}'..='\u{a9b9}')
        .separators('\u{a9bc}'..='\u{a9bd}')
        .separator('\u{a9e5}')
        .separators('\u{aa29}'..='\u{aa2e}')
        .separators('\u{aa31}'..='\u{aa32}')
        .separators('\u{aa35}'..='\u{aa36}')
        .separator('\u{aa43}')
        .separator('\u{aa4c}')
        .separator('\u{aa7c}')
        .separator('\u{aab0}')
        .separators('\u{aab2}'..='\u{aab4}')
        .separators('\u{aab7}'..='\u{aab8}')
        .separators('\u{aabe}'..='\u{aabf}')
        .separator('\u{aac1}')
        .separators('\u{aaec}'..='\u{aaed}')
        .separator('\u{aaf6}')
        .separator('\u{abe5}')
        .separator('\u{abe8}')
        .separator('\u{abed}')
        .separator('\u{fb1e}')
        .separators('\u{fe00}'..='\u{fe0f}')
        .separators('\u{fe20}'..='\u{fe2f}')
        .separator('\u{101fd}')
        .separator('\u{102e0}')
        .separators('\u{10376}'..='\u{1037a}')
        .separators('\u{10a01}'..='\u{10a03}')
        .separators('\u{10a05}'..='\u{10a06}')
        .separators('\u{10a0c}'..='\u{10a0f}')
        .separators('\u{10a38}'..='\u{10a3a}')
        .separator('\u{10a3f}')
        .separators('\u{10ae5}'..='\u{10ae6}')
        .separators('\u{10d24}'..='\u{10d27}')
        .separators('\u{10eab}'..='\u{10eac}')
        .separators('\u{10f46}'..='\u{10f50}')
        .separator('\u{11001}')
        .separators('\u{11038}'..='\u{11046}')
        .separators('\u{1107f}'..='\u{11081}')
        .separators('\u{110b3}'..='\u{110b6}')
        .separators('\u{110b9}'..='\u{110ba}')
        .separators('\u{11100}'..='\u{11102}')
        .separators('\u{11127}'..='\u{1112b}')
        .separators('\u{1112d}'..='\u{11134}')
        .separator('\u{11173}')
        .separators('\u{11180}'..='\u{11181}')
        .separators('\u{111b6}'..='\u{111be}')
        .separators('\u{111c9}'..='\u{111cc}')
        .separator('\u{111cf}')
        .separators('\u{1122f}'..='\u{11231}')
        .separator('\u{11234}')
        .separators('\u{11236}'..='\u{11237}')
        .separator('\u{1123e}')
        .separator('\u{112df}')
        .separators('\u{112e3}'..='\u{112ea}')
        .separators('\u{11300}'..='\u{11301}')
        .separators('\u{1133b}'..='\u{1133c}')
        .separator('\u{11340}')
        .separators('\u{11366}'..='\u{1136c}')
        .separators('\u{11370}'..='\u{11374}')
        .separators('\u{11438}'..='\u{1143f}')
        .separators('\u{11442}'..='\u{11444}')
        .separator('\u{11446}')
        .separator('\u{1145e}')
        .separators('\u{114b3}'..='\u{114b8}')
        .separator('\u{114ba}')
        .separators('\u{114bf}'..='\u{114c0}')
        .separators('\u{114c2}'..='\u{114c3}')
        .separators('\u{115b2}'..='\u{115b5}')
        .separators('\u{115bc}'..='\u{115bd}')
        .separators('\u{115bf}'..='\u{115c0}')
        .separators('\u{115dc}'..='\u{115dd}')
        .separators('\u{11633}'..='\u{1163a}')
        .separator('\u{1163d}')
        .separators('\u{1163f}'..='\u{11640}')
        .separator('\u{116ab}')
        .separator('\u{116ad}')
        .separators('\u{116b0}'..='\u{116b5}')
        .separator('\u{116b7}')
        .separators('\u{1171d}'..='\u{1171f}')
        .separators('\u{11722}'..='\u{11725}')
        .separators('\u{11727}'..='\u{1172b}')
        .separators('\u{1182f}'..='\u{11837}')
        .separators('\u{11839}'..='\u{1183a}')
        .separators('\u{1193b}'..='\u{1193c}')
        .separator('\u{1193e}')
        .separator('\u{11943}')
        .separators('\u{119d4}'..='\u{119d7}')
        .separators('\u{119da}'..='\u{119db}')
        .separator('\u{119e0}')
        .separators('\u{11a01}'..='\u{11a0a}')
        .separators('\u{11a33}'..='\u{11a38}')
        .separators('\u{11a3b}'..='\u{11a3e}')
        .separator('\u{11a47}')
        .separators('\u{11a51}'..='\u{11a56}')
        .separators('\u{11a59}'..='\u{11a5b}')
        .separators('\u{11a8a}'..='\u{11a96}')
        .separators('\u{11a98}'..='\u{11a99}')
        .separators('\u{11c30}'..='\u{11c36}')
        .separators('\u{11c38}'..='\u{11c3d}')
        .separator('\u{11c3f}')
        .separators('\u{11c92}'..='\u{11ca7}')
        .separators('\u{11caa}'..='\u{11cb0}')
        .separators('\u{11cb2}'..='\u{11cb3}')
        .separators('\u{11cb5}'..='\u{11cb6}')
        .separators('\u{11d31}'..='\u{11d36}')
        .separator('\u{11d3a}')
        .separators('\u{11d3c}'..='\u{11d3d}')
        .separators('\u{11d3f}'..='\u{11d45}')
        .separator('\u{11d47}')
        .separators('\u{11d90}'..='\u{11d91}')
        .separator('\u{11d95}')
        .separator('\u{11d97}')
        .separators('\u{11ef3}'..='\u{11ef4}')
        .separators('\u{16af0}'..='\u{16af4}')
        .separators('\u{16b30}'..='\u{16b36}')
        .separator('\u{16f4f}')
        .separators('\u{16f8f}'..='\u{16f92}')
        .separator('\u{16fe4}')
        .separators('\u{1bc9d}'..='\u{1bc9e}')
        .separators('\u{1d167}'..='\u{1d169}')
        .separators('\u{1d17b}'..='\u{1d182}')
        .separators('\u{1d185}'..='\u{1d18b}')
        .separators('\u{1d1aa}'..='\u{1d1ad}')
        .separators('\u{1d242}'..='\u{1d244}')
        .separators('\u{1da00}'..='\u{1da36}')
        .separators('\u{1da3b}'..='\u{1da6c}')
        .separator('\u{1da75}')
        .separator('\u{1da84}')
        .separators('\u{1da9b}'..='\u{1da9f}')
        .separators('\u{1daa1}'..='\u{1daaf}')
        .separators('\u{1e000}'..='\u{1e006}')
        .separators('\u{1e008}'..='\u{1e018}')
        .separators('\u{1e01b}'..='\u{1e021}')
        .separators('\u{1e023}'..='\u{1e024}')
        .separators('\u{1e026}'..='\u{1e02a}')
        .separators('\u{1e130}'..='\u{1e136}')
        .separators('\u{1e2ec}'..='\u{1e2ef}')
        .separators('\u{1e8d0}'..='\u{1e8d6}')
        .separators('\u{1e944}'..='\u{1e94a}')
        .separators('\u{e0100}'..='\u{e01ef}')
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
            // Space modifier letters.
            ("h", "ʰ"),
            ("h", "ʱ"),
            ("j", "ʲ"),
            ("L", "˪"),
            ("r", "ʳ"),
            ("s", "ˢ"),
            ("w", "ʷ"),
            ("x", "˟"),
            ("x", "ˣ"),
            ("y", "ʸ"),
            // Emoji.
            ("e", "📧"),
            ("h", "♓"),
            ("i", "ℹ"),
            ("id", "🆔"),
            ("m", "Ⓜ"),
            ("u", "⛎"),
            ("v", "♈"),
            ("z", "Ⓩ"),
        ]);

    // Profane.
    if cfg!(feature = "damn") {
        generator
            .words(&["damn", "damnit", "goddamn", "goddamnit"])
            .aliases(&[("damnit", "dammit")]);
    }

    if cfg!(feature = "hell") {
        generator.words(&["hell", "hella", "hells"]).exceptions(&[
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
        generator
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
        generator
            .words(&[
                "ass",
                "asses",
                "asshole",
                "assholes",
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
        generator.words(&["blowjob", "blowjobs"]);
    }

    if cfg!(feature = "cunt") {
        generator.words(&["cunt", "cunts"]);
    }

    if cfg!(feature = "dick") {
        generator
            .words(&[
                "dick",
                "dickaround",
                "dicked",
                "dickhead",
                "dickheads",
                "dicking",
                "dicks",
                "dickup",
            ])
            .exceptions(&[
                "benedick",
                "dickcissel",
                "dickens",
                "dicker",
                "dickey",
                "dickie",
                "dicky",
                "medick",
                "zaddick",
            ]);
    }

    if cfg!(feature = "fuck") {
        generator
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

    if cfg!(feature = "penis") {
        generator
            .words(&["macropenis", "micropenis", "penis", "penises", "penishead"])
            .exceptions(&["karpenisi", "penisterophily", "penistone", "scherpenisse"]);
    }

    if cfg!(feature = "shit") {
        generator.words(&[
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

    if cfg!(feature = "vagina") {
        generator.words(&["vagina", "vaginas"]);
    }

    // Offensive.
    if cfg!(feature = "bastard") {
        generator
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
        generator.words(&[
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
        generator.words(&[
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
        generator
            .words(&["nigger", "niggers"])
            .exception("snigger")
            .aliases(&[("nigger", "nibber")]);
    }

    if cfg!(feature = "slut") {
        generator.words(&[
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
        generator.words(&["🖕"]).aliases(&[
            ("🖕", "🖕🏻"),
            ("🖕", "🖕🏼"),
            ("🖕", "🖕🏽"),
            ("🖕", "🖕🏾"),
            ("🖕", "🖕🏿"),
        ]);
    }

    writeln!(&mut file, "{}", generator.generate("PROFANITY_FILTER")).unwrap();
}
