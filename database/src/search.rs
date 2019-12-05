use std::collections::{HashMap, HashSet};

use crate::models::{Word, WordPronunciation};
use crate::DbConnection;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1, take_while_m_n};
use nom::character::complete::anychar;
use nom::combinator::map;
use nom::error::{ErrorKind, ParseError};
use nom::multi::many0;
use nom::sequence::pair;
use nom::{Err, IResult};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

type Id = i32;

impl Error for SearchError {}

#[derive(Debug)]
pub enum SearchError {
    InvalidInput,
}

impl Display for SearchError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::InvalidInput => write!(fmt, "Invalid query")
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum QueryToken {
    WildcardSingle,
    Character(char),
    Pronunciation { sound: String, tone: String },
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum PronunciationType {
    Pinyin,
    Jyutping,
}

impl PronunciationType {
    fn from_integer(i: i32) -> Option<Self> {
        match i {
            0 => Some(Self::Pinyin),
            1 => Some(Self::Jyutping),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Syllable {
    sound: String,
    tone: String,
}

#[derive(Hash, Debug)]
struct Characters {
    simplified: String,
    traditional: String,
}

pub struct DictSearch {
    pronunciation_reverse_map: HashMap<(PronunciationType, Syllable), HashSet<Id>>,
    pronunciation_map: HashMap<(PronunciationType, Id), Vec<Syllable>>,
    // "character" here means a Chinese character (字)
    character_reverse_map: HashMap<char, HashSet<Id>>,
    characters_map: HashMap<Id, Characters>,
}

impl DictSearch {
    pub fn new() -> Self {
        Self {
            pronunciation_reverse_map: HashMap::new(),
            pronunciation_map: HashMap::new(),
            character_reverse_map: HashMap::new(),
            characters_map: HashMap::new(),
        }
    }
    pub fn insert_pronunciation(
        &mut self,
        id: Id,
        content: &str,
        pronunciation_type: PronunciationType,
    ) {
        // todo. some entries have no spaces in between syllables
        let (_, tokens) = tokenise_pronunciation(content).unwrap();

        for token in &tokens {
            self.pronunciation_reverse_map
                .entry((pronunciation_type, token.clone()))
                .or_insert_with(|| HashSet::new())
                .insert(id);
        }
        self.pronunciation_map.insert((pronunciation_type, id), tokens);
    }
    pub fn insert_characters(&mut self, id: Id, trad: &str, simp: &str) {
        for character in simp.chars() {
            self.character_reverse_map
                .entry(character.clone())
                .or_insert_with(|| HashSet::new())
                .insert(id);
        }
        for character in trad.chars() {
            self.character_reverse_map
                .entry(character.clone())
                .or_insert_with(|| HashSet::new())
                .insert(id);
        }
        self.characters_map.insert(
            id,
            Characters {
                simplified: simp.to_string(),
                traditional: trad.to_string(),
            },
        );
    }
    pub fn search_pronunciation(
        &self,
        query: &str,
        pronunciation_type: PronunciationType,
    ) -> Result<Vec<Id>, SearchError> {
        let s = tokenise_pronunciation_query(&query);
        let query_tokens = s.map_err(|_| SearchError::InvalidInput)?.1;

        let mut searches = Vec::new();

        // Apply a broad filter by filtering by the first
        // non-wildcard token in the search query.
        let mut broad_filter = None;
        for token in &query_tokens {
            if let QueryToken::Pronunciation { sound, tone } = token {
                broad_filter = Some((sound, tone));
                break;
            }
        }

        // if tone is empty
        if let Some((sound, tone)) = broad_filter {
            // if tone unspecified
            if tone == "" {
                // consider all tones
                for i in 1..7 {
                    searches.push(Syllable {
                        sound: sound.clone(),
                        tone: i.to_string(),
                    });
                }
            } else {
                searches.push(Syllable {
                    sound: sound.clone(),
                    tone: tone.clone(),
                });
            }
        } else {
            return Err(SearchError::InvalidInput);
        }

        let mut results = Vec::new();
        // broad phase filter according to the first non-wildcard syllable in the query
        for search in searches {
            if let Some(pronunciation_ids) = self
                .pronunciation_reverse_map
                .get(&(pronunciation_type, search))
            {
                for id in pronunciation_ids {
                    let candidate = self.pronunciation_map.get(&(pronunciation_type, *id)).unwrap();
                    // is the candidate valid?
                    let mut good = true;

                    for (i, token) in query_tokens.iter().enumerate() {
                        if let QueryToken::Pronunciation { sound, tone } = token {
                            // fetch token from candidate.
                            let candidate_syllable = match candidate.get(i) {
                                Some(t) => t,
                                None => {
                                    good = false;
                                    break;
                                }
                            };
                            // check if syllable matches.
                            if &candidate_syllable.sound != sound
                                || (tone != "" && &candidate_syllable.tone != tone)
                            {
                                good = false;
                                break;
                            }
                        }
                        if let QueryToken::WildcardSingle = token {
                            // check that there is actually a syllable.
                            if let None = candidate.get(i) {
                                good = false;
                                break;
                            }
                        }
                    }
                    if good {
                        results.push(*id);
                    }
                }
            }
        }
        Ok(results)
    }

    pub fn search_characters(&self, query: &str) -> Result<Vec<Id>, SearchError> {
        let s = tokenise_characters_query(&query);
        let query_tokens = s.map_err(|_| SearchError::InvalidInput)?.1;

        let mut broad_filter_option = None;
        for token in &query_tokens {
            if let QueryToken::Character(c) = token {
                broad_filter_option = Some(c);
                break;
            };
        }

        let broad_filter = broad_filter_option.ok_or(SearchError::InvalidInput)?;
        let mut results = Vec::new();

        if let Some(word_ids) = self.character_reverse_map.get(broad_filter) {
            for id in word_ids {
                let candidate = self.characters_map.get(&id).unwrap();
                let mut valid = true;

                for (i, token) in query_tokens.iter().enumerate() {
                    if let QueryToken::Character(query_char) = token {
                        // allow simplified or traditional variants for each character.
                        let trad_char = match candidate.traditional.chars().nth(i) {
                            Some(t) => t,
                            None => {
                                valid = false;
                                break;
                            }
                        };
                        let simp_char = match candidate.simplified.chars().nth(i) {
                            Some(t) => t,
                            None => {
                                valid = false;
                                break;
                            }
                        };
                        if query_char != &trad_char && query_char != &simp_char {
                            valid = false;
                            break;
                        }
                    } else if let QueryToken::WildcardSingle = token {
                        // check that there is actually a syllable.
                        if candidate.traditional.chars().nth(i).is_none()
                            && candidate.simplified.chars().nth(i).is_none()
                        {
                            valid = false;
                            break;
                        }
                    }
                }

                if valid {
                    results.push(*id);
                }
            }
        }
        Ok(results)
    }
}

/// Load database data into the search index.
pub fn load_search(search: &mut DictSearch, connection: &DbConnection) {
    use crate::diesel::prelude::*;
    use crate::schema::{word::dsl::*, word_pronunciation::dsl::*};

    let results = word_pronunciation
        .load::<WordPronunciation>(connection)
        .unwrap();
    let words = word.load::<Word>(connection).unwrap();

    for result in results {
        search.insert_pronunciation(
            result.pronunciation_id,
            &result.pronunciation,
            PronunciationType::from_integer(result.pronunciation_type).unwrap(),
        );
    }
    for word_result in words {
        search.insert_characters(
            word_result.word_id,
            &word_result.traditional,
            &word_result.simplified,
        );
    }
}

fn tokenise_pronunciation(content: &str) -> IResult<&str, Vec<Syllable>> {
    separated_list_custom(
        // spaces and unknown characters
        take_while(|c: char| !c.is_ascii_alphanumeric()),
        map(
            pair(
                take_while1(|c: char| c.is_ascii_alphabetic()),
                take_while_m_n(0, 1, |c: char| c.is_ascii_digit()),
            ),
            |(x, y): (&str, &str)| Syllable {
                sound: x.to_string(),
                tone: y.to_string(),
            },
        ),
    )(content)
}

fn tokenise_pronunciation_query(query: &str) -> IResult<&str, Vec<QueryToken>> {
    separated_list_custom(
        // spaces and unknown characters
        take_while(|c: char| !c.is_ascii_alphanumeric() && c != '?'),
        alt((
            map(tag("?"), |_| QueryToken::WildcardSingle),
            map(
                pair(
                    take_while1(|c: char| c.is_ascii_alphabetic()),
                    take_while_m_n(0, 1, |c: char| c.is_ascii_digit()),
                ),
                |(x, y): (&str, &str)| QueryToken::Pronunciation {
                    sound: x.to_string(),
                    tone: y.to_string(),
                },
            ),
        )),
    )(query)
}
fn tokenise_characters_query(query: &str) -> IResult<&str, Vec<QueryToken>> {
    many0(
        // spaces and unknown characters
        alt((
            map(tag("?"), |_| QueryToken::WildcardSingle),
            map(anychar, |c| QueryToken::Character(c)),
        )),
    )(query)
}

// Modified from nom separated_list
pub fn separated_list_custom<I, O, O2, E, F, G>(sep: G, f: F) -> impl Fn(I) -> IResult<I, Vec<O>, E>
where
    I: Clone + PartialEq,
    F: Fn(I) -> IResult<I, O, E>,
    G: Fn(I) -> IResult<I, O2, E>,
    E: ParseError<I>,
{
    move |i: I| {
        let mut res = Vec::new();
        let mut i = i.clone();

        match f(i.clone()) {
            Err(Err::Error(_)) => return Ok((i, res)),
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                if i1 == i {
                    return Err(Err::Error(E::from_error_kind(i1, ErrorKind::SeparatedList)));
                }

                res.push(o);
                i = i1;
            }
        }

        loop {
            let mut empty_sep = false;
            match sep(i.clone()) {
                Err(Err::Error(_)) => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // Special modification. We will return an error
                    // only if the separator and element are both empty.
                    if i1 == i {
                        empty_sep = true;
                    }

                    match f(i1.clone()) {
                        Err(Err::Error(_)) => return Ok((i, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            if i2 == i {
                                if empty_sep {
                                    return Err(Err::Error(E::from_error_kind(
                                        i2,
                                        ErrorKind::SeparatedList,
                                    )));
                                }
                            }

                            res.push(o);
                            i = i2;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn p(x: &str, y: &str) -> Syllable {
        Syllable {
            sound: x.to_string(),
            tone: y.to_string(),
        }
    }
    fn qc(x: char) -> QueryToken {
        QueryToken::Character(x)
    }
    fn qp(x: &str, y: &str) -> QueryToken {
        QueryToken::Pronunciation {
            sound: x.to_string(),
            tone: y.to_string(),
        }
    }
    #[test]
    fn pronunciation_strange_chinese_character() {
        assert_eq!(
            tokenise_pronunciation("jat1 檔:dong2"),
            Ok(("", vec![p("jat", "1"), p("dong", "2")]))
        );
    }
    #[test]
    fn pronunciation_your_head() {
        assert_eq!(
            tokenise_pronunciation("nei5 go3 tau4"),
            Ok(("", vec![p("nei", "5"), p("go", "3"), p("tau", "4")]))
        );
    }
    #[test]
    fn pronunciation_rubbish_bin() {
        assert_eq!(
            tokenise_pronunciation("laap6saap3 tung2"),
            Ok(("", vec![p("laap", "6"), p("saap", "3"), p("tung", "2")]))
        );
    }
    // strange entry
    #[test]
    fn pronunciation_set_tou() {
        assert_eq!(
            tokenise_pronunciation("s e t tou2"),
            Ok(("", vec![p("s", ""), p("e", ""), p("t", ""), p("tou", "2")]))
        );
    }

    #[test]
    fn pronunciation_query_rubbish_bin() {
        assert_eq!(
            tokenise_pronunciation_query("laap6 ? tung2"),
            Ok((
                "",
                vec![qp("laap", "6"), QueryToken::WildcardSingle, qp("tung", "2")]
            ))
        );
    }

    #[test]
    fn chars_query_sit() {
        assert_eq!(
            tokenise_characters_query("sit"),
            Ok(("", vec![qc('s'), qc('i'), qc('t')]))
        );
    }
    #[test]
    fn chars_query_rubbish_bin() {
        assert_eq!(
            tokenise_characters_query("垃?桶"),
            Ok(("", vec![qc('垃'), QueryToken::WildcardSingle, qc('桶')]))
        );
    }
}
