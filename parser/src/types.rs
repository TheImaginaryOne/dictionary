#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub traditional: String,
    pub simplified: String,
    pub dictionary_id: i32,
    pub pinyin: String,
    pub jyutping: String,
    pub definition: String,
}

impl Entry {
    pub fn new(
        traditional: &str,
        simplified: &str,
        pinyin: &str,
        jyutping: &str,
        definition: &str,
        dictionary_id: i32,
    ) -> Self {
        Self {
            traditional: traditional.into(),
            simplified: simplified.into(),
            pinyin: pinyin.into(),
            jyutping: jyutping.into(),
            definition: definition.into(),
            dictionary_id,
        }
    }
}
