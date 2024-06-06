use csv::{
    recorder::{Into as IntoRecorder, Options as RecorderOptions},
    tokenizer::{
        options::{Builder as TokenizerOptionsBuilder, LineBreak},
        Into as IntoCsvTokenizer, Options as TokenizerOptions,
    },
};
use itertools::Itertools;

fn main() {
    const TEXT: &str = r"field1,field2,field3
field4
";
    let mut tokens = match TEXT
        .chars()
        .csv_tokens_custom(TokenizerOptions::default().with_line_break(LineBreak::Lf))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(tokens) => tokens.into_iter(),
        Err(cause) => panic!("error {cause}"),
    };
    println!("{tokens:#?}");
    println!();

    let recorder_options = RecorderOptions::default();

    loop {
        let record = (&mut tokens)
            .csv_record_custom(&recorder_options)
            .collect_vec();
        println!("{record:?}");
        if record.is_empty()
            || record.len() == 1 && record[0].as_ref().is_ok_and(std::string::String::is_empty)
        {
            break;
        }
    }
}
