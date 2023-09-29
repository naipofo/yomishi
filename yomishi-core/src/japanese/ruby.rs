use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Segment {
    Text(String),
    Ruby(String, String),
}

fn is_kanji(c: char) -> bool {
    ('\u{3000}'..='\u{303F}').contains(&c) || ('\u{4E00}'..='\u{9FEF}').contains(&c)
}

fn build_push(pairs: &mut Vec<Segment>, kanji_build: &mut String, kana_build: &mut String) {
    pairs.push(if kana_build.is_empty() {
        Segment::Text(kanji_build.clone())
    } else {
        Segment::Ruby(kanji_build.clone(), kana_build.clone())
    });
    kanji_build.clear();
    kana_build.clear();
}
// TODO: Operate on &str instead of String

/// Constructs a ruby sequence
///
/// # Example
/// ```
/// let ruby = vec![
///     Segment::Text("お".to_string()),
///     Segment::Ruby("欠".to_string(), "か".to_string()),
///     Segment::Text("かす".to_string()),
/// ];
/// assert_eq!(
///     try_from_reading("お欠かす".to_string(), "おかかす".to_string()),
///     ruby
/// );
/// ```
pub fn try_from_reading(expression: String, reading: String) -> Vec<Segment> {
    let mut pairs = Vec::new();

    let kanji = expression.chars().collect::<Vec<_>>();
    let kana = reading.chars().collect::<Vec<_>>();

    let mut rp = 0;
    let mut kana_build = String::new();
    let mut kanji_build = String::new();

    for (kp, kanji_char) in kanji.iter().enumerate() {
        kanji_build.push(*kanji_char);

        if kp + 1 == kanji.len() {
            while rp < kana.len() {
                kana_build.push(kana[rp]);
                rp += 1;
            }

            if !is_kanji(*kanji_char) {
                kana_build.clear();
            }

            build_push(&mut pairs, &mut kanji_build, &mut kana_build);
            break;
        } else if is_kanji(*kanji_char) && !is_kanji(kanji[kp + 1]) {
            while rp < kana.len()
                && (kanji[kp + 1] != kana[rp]
                    || kana_build.chars().count() < kanji_build.chars().count())
            {
                kana_build.push(kana[rp]);
                rp += 1;
            }

            build_push(&mut pairs, &mut kanji_build, &mut kana_build);
        } else if !is_kanji(*kanji_char) && is_kanji(kanji[kp + 1]) {
            kana_build.clear();
            rp += kanji_build.chars().count();
            build_push(&mut pairs, &mut kanji_build, &mut kana_build);
        }
    }

    pairs
}
