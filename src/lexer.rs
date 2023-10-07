use logos::Logos;

use crate::reader::{FileType, GossipFile};

pub struct EnrichedToken {
    pub token: Token,
    pub slice: String,
}

pub struct TokenisedFile {
    pub file_type: FileType,
    pub tokens: Vec<EnrichedToken>,
}

// #[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("[")]
    LeftSquareBracket,

    #[token("]")]
    RightSquareBracket,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(":")]
    Colon,

    #[token("->")]
    Arrow,

    #[token("@")]
    At,

    #[token("\n")]
    NewLine,

    #[token("begin")]
    Begin,

    #[regex("[a-zA-Z0-9_ ]+", priority = 2)]
    Identifier,

    #[regex("[a-zA-Z0-9 !@#$%&*.?;]+")]
    Text,
}

fn get_tokens(contents: String, file_name: String) -> Vec<EnrichedToken> {
    let mut tokens = Vec::new();
    let mut lex = Token::lexer(&contents);
    loop {
        match lex.next() {
            Some(Ok(token)) => tokens.push(EnrichedToken {
                token,
                slice: lex.slice().to_string(),
            }),
            Some(Err(_)) => panic!("Lexing Error!"),
            None => break,
        }
    }
    return tokens;
}

pub fn get_tokenised_files(
    files: impl Iterator<Item = GossipFile>,
) -> impl Iterator<Item = TokenisedFile> {
    return files.map(|f| TokenisedFile {
        file_type: f.file_type,
        tokens: get_tokens(f.contents, f.file_path),
    });
}
