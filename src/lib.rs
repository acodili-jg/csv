use recorder::Into;
use tokenizer::Into as IntoTokenizer;

pub mod recorder;
pub mod token;
pub mod tokenizer;

/// # Errors
///
/// TODO
pub fn parse(s: &str) -> Result<Vec<Vec<String>>, ParseError> {
    let mut tokens = s
        .chars()
        .csv_tokens()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter();
    let mut records = Vec::new();
    let mut len = None;
    loop {
        let record = (&mut tokens).csv_record().collect::<Result<Vec<_>, _>>()?;
        if record.is_empty() {
            break;
        }
        match len {
            Some(len) if len != record.len() => {
                return Err(ParseError::JaggedRecords {
                    expected: len,
                    at: records.len(),
                    actual: record.len(),
                })
            }
            Some(_) => {}
            None => len = Some(record.len()),
        }
        records.push(record);
    }
    Ok(records)
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ParseError {
    #[error("{0}")]
    Tokenizer(#[from] tokenizer::Error),
    #[error("{0}")]
    Recorder(#[from] recorder::Error),
    #[error("expected a uniform length of {expected} at {at}, instead got {actual}")]
    JaggedRecords {
        expected: usize,
        at: usize,
        actual: usize,
    },
}
