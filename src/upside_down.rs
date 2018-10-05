// This file is part of "upside-down", which is free software: you
// can redistribute it and/or modify it under the terms of the GNU General
// Public License version 3 as published by the Free Software Foundation. See
// <https://www.gnu.org/licenses/> for a copy.

// based on https://www.fileformat.info/convert/text/upside-down-map.htm
const SWAPLENGTH: usize = 114;
static SWAPLIST: [char; SWAPLENGTH] = [
    '!', '"', '&', '\'', '(', '.', '3', '4', '6', '7', ';', '<', '?', 'A', 'B', 'C', 'D', 'E', 'F',
    'G', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'T', 'U', 'V', 'Y', '[', '_', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'r', 't', 'v', 'w', 'y', '{', '‿',
    '⁅', '∴', '¡', '„', '⅋', ',', ')', '˙', 'Ɛ', 'ᔭ', '9', 'Ɫ', '؛', '>', '¿',
    '∀', '𐐒', 'Ↄ', '◖', 'Ǝ', 'Ⅎ', '⅁', 'ſ', '⋊', '⅂', 'W', 'ᴎ', 'Ԁ', 'Ό',
    'ᴚ', '⊥', '∩', 'ᴧ', '⅄', ']', '‾', 'ɐ', 'q', 'ɔ', 'p', 'ǝ', 'ɟ', 'ƃ', 'ɥ',
    'ı', 'ɾ', 'ʞ', 'ʃ', 'ɯ', 'u', 'ɹ', 'ʇ', 'ʌ', 'ʍ', 'ʎ', '}', '⁀', '⁆',
    '∵',
];
/// This will flip the text upside down and reverse the order so that it is somewhat readable
pub fn upside_down(original: &str) -> String {
    original
        .chars()
        .map(|c| match SWAPLIST.iter().position(|&c2| c == c2) {
            Some(i) => SWAPLIST[(i + SWAPLENGTH / 2) % SWAPLENGTH],
            None => c,
        })
        .rev()
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_string_lowercase() {
        assert_eq!(
            String::from("Ɫ9ᔭƐ˙),⅋„¡"),
            upside_down("!\"&'(.3467")
        );
    }
    #[test]
    fn test_string_uppercase() {
        assert_eq!(
            String::from("Z⅄Xᴧ∩⊥SᴚΌԀOᴎW⅂⋊ſIH⅁ℲƎ◖Ↄ𐐒∀"),
            upside_down("ABCDEFGHIJKLMNOPQRSTUVXYZ")
        );
    }
}
