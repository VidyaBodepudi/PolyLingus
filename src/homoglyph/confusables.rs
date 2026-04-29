use std::collections::HashMap;

/// Core confusables database — maps Latin characters to their Unicode lookalikes
/// Based on Unicode TR39 confusables data
pub fn get_confusables() -> HashMap<char, Vec<char>> {
    let mut map = HashMap::new();
    // Latin → Cyrillic
    map.insert('a', vec!['а', 'ɑ', 'α']); // Cyrillic а, Latin alpha, Greek alpha
    map.insert('c', vec!['с', 'ϲ']);       // Cyrillic с, Greek lunate sigma
    map.insert('d', vec!['ԁ', 'ɗ']);       // Cyrillic palochka variants
    map.insert('e', vec!['е', 'ё']);        // Cyrillic е
    map.insert('h', vec!['һ', 'ℎ']);       // Cyrillic shha, planck constant
    map.insert('i', vec!['і', 'ɪ', 'ⅰ']); // Cyrillic і, small cap I, Roman numeral
    map.insert('j', vec!['ј', 'ʝ']);       // Cyrillic je
    map.insert('k', vec!['к', 'ⲕ']);       // Cyrillic к, Coptic kapa
    map.insert('l', vec!['ⅼ', 'ℓ', 'ǀ']); // Roman numeral, script l, click
    map.insert('m', vec!['м', 'ⅿ']);       // Cyrillic м, Roman numeral
    map.insert('n', vec!['ո', 'ṇ']);       // Armenian vo
    map.insert('o', vec!['о', 'ο', '০', 'ᴏ']); // Cyrillic о, Greek omicron, Bengali 0
    map.insert('p', vec!['р', 'ρ']);       // Cyrillic р, Greek rho
    map.insert('q', vec!['ԛ', 'ɋ']);       // Cyrillic qa
    map.insert('s', vec!['ѕ', 'ꜱ']);       // Cyrillic dze, small cap s
    map.insert('t', vec!['т', 'ṭ']);       // Cyrillic т
    map.insert('u', vec!['υ', 'ս']);       // Greek upsilon, Armenian seh
    map.insert('v', vec!['ν', 'ⅴ']);       // Greek nu, Roman numeral
    map.insert('w', vec!['ω', 'ԝ']);       // Greek omega, Cyrillic we
    map.insert('x', vec!['х', 'ⅹ', 'χ']); // Cyrillic kha, Roman X, Greek chi
    map.insert('y', vec!['у', 'γ']);        // Cyrillic у, Greek gamma
    map.insert('z', vec!['ᴢ', 'ζ']);       // Small cap z, Greek zeta

    // Uppercase
    map.insert('A', vec!['А', 'Α', 'Ꭺ']); // Cyrillic А, Greek Alpha, Cherokee
    map.insert('B', vec!['В', 'Β', 'Ꞵ']); // Cyrillic В, Greek Beta
    map.insert('C', vec!['С', 'Ϲ', 'Ꮯ']); // Cyrillic С, Greek lunate sigma
    map.insert('D', vec!['Ⅾ', 'Ꭰ']);      // Roman D, Cherokee
    map.insert('E', vec!['Е', 'Ε']);       // Cyrillic Е, Greek Epsilon
    map.insert('H', vec!['Н', 'Η']);       // Cyrillic Н, Greek Eta
    map.insert('I', vec!['І', 'Ⅰ', 'Ι']); // Cyrillic І, Roman I, Greek Iota
    map.insert('J', vec!['Ј', 'Ꭻ']);      // Cyrillic Ј
    map.insert('K', vec!['К', 'Κ', 'Ꮶ']); // Cyrillic К, Greek Kappa
    map.insert('L', vec!['Ⅼ', 'Ꮮ']);      // Roman L
    map.insert('M', vec!['М', 'Μ', 'Ⅿ']); // Cyrillic М, Greek Mu, Roman M
    map.insert('N', vec!['Ν', 'Ꮑ']);      // Greek Nu
    map.insert('O', vec!['О', 'Ο', 'Ꮎ']); // Cyrillic О, Greek Omicron
    map.insert('P', vec!['Р', 'Ρ']);       // Cyrillic Р, Greek Rho
    map.insert('S', vec!['Ѕ', 'Ꮪ']);      // Cyrillic Ѕ
    map.insert('T', vec!['Т', 'Τ', 'Ꭲ']); // Cyrillic Т, Greek Tau
    map.insert('V', vec!['Ⅴ', 'Ꮩ']);      // Roman V
    map.insert('W', vec!['Ꮃ', 'Ԝ']);      // Cherokee
    map.insert('X', vec!['Х', 'Χ', 'Ⅹ']); // Cyrillic Х, Greek Chi, Roman X
    map.insert('Y', vec!['У', 'Υ', 'Ꭹ']); // Cyrillic У, Greek Upsilon
    map.insert('Z', vec!['Ζ', 'Ꮓ']);      // Greek Zeta

    // Digits
    map.insert('0', vec!['О', 'о', 'Ο', 'ο', '０']); // Cyrillic/Greek O, fullwidth
    map.insert('1', vec!['Ⅰ', 'ⅰ', 'ℓ', '１']);
    map.insert('3', vec!['Ʒ', 'ʒ', '３']);
    map.insert('5', vec!['Ѕ', 'ѕ', '５']);
    map.insert('8', vec!['Ȣ', '８']);

    map
}

/// Get the Unicode script category for a character
pub fn get_script(c: char) -> &'static str {
    match c as u32 {
        0x0400..=0x04FF | 0x0500..=0x052F => "Cyrillic",
        0x0370..=0x03FF | 0x1F00..=0x1FFF => "Greek",
        0x0530..=0x058F => "Armenian",
        0x10A0..=0x10FF | 0x2D00..=0x2D2F => "Georgian",
        0x13A0..=0x13FF | 0xAB70..=0xABBF => "Cherokee",
        0x2C80..=0x2CFF => "Coptic",
        0x1680..=0x169F => "Ogham",
        0x16A0..=0x16FF => "Runic",
        0x0000..=0x007F => "Latin",
        0x0080..=0x024F => "Latin Extended",
        _ => "Other",
    }
}
