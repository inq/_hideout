#[derive(Debug)]
pub struct FormEntry {
    name: String,
    value: String,
}

#[derive(Debug)]
pub struct FormData {
    data: Vec<FormEntry>,
}

#[derive(Debug)]
enum State {
    Normal,
    HasName(String),
}

#[derive(Debug)]
enum WordState {
    Normal,
    Empty,
    Value(u8),
}

// TODO: Give detailed error
#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "invalid input")]
    InvalidInput,
}

impl FormData {
    pub fn parse_x_www_form_urlencoded(input: &[u8]) -> Result<FormData, failure::Error> {
        let mut res = vec![];
        let mut buf = vec![];
        let mut state = State::Normal;
        let mut word_state = WordState::Normal;
        for c in input {
            match (&mut state, &word_state, c) {
                (State::Normal, WordState::Normal, b'=') => {
                    state = State::HasName(String::from_utf8(std::mem::take(&mut buf))?);
                }
                (State::Normal, WordState::Normal, b'&') => {
                    let name = String::from_utf8(std::mem::take(&mut buf))?;
                    res.push(FormEntry {
                        name,
                        value: "".to_string(),
                    });
                }
                (State::HasName(name), WordState::Normal, b'&') => {
                    let value = String::from_utf8(std::mem::take(&mut buf))?;
                    res.push(FormEntry {
                        name: std::mem::take(name),
                        value,
                    });
                    state = State::Normal;
                }
                (_, WordState::Normal, b'%') => {
                    word_state = WordState::Empty;
                }
                (_, WordState::Normal, c) => {
                    buf.push(*c);
                }
                (_, WordState::Empty, c) => {
                    if let Some(num) = (*c as char).to_digit(16) {
                        word_state = WordState::Value(num as u8);
                    } else {
                        return Err(Error::InvalidInput.into());
                    }
                }
                (_, WordState::Value(parent), c) => {
                    if let Some(num) = (*c as char).to_digit(16) {
                        buf.push(parent * 16 + num as u8);
                        word_state = WordState::Normal;
                    } else {
                        return Err(Error::InvalidInput.into());
                    }
                }
            }
        }
        match (state, word_state) {
            (State::Normal, WordState::Normal) => {
                let name = String::from_utf8(std::mem::take(&mut buf))?;
                res.push(FormEntry {
                    name,
                    value: "".to_string(),
                });
            }
            (State::HasName(name), WordState::Normal) => {
                let value = String::from_utf8(std::mem::take(&mut buf))?;
                res.push(FormEntry { name, value });
            }
            _ => {
                return Err(Error::InvalidInput.into());
            }
        }
        Ok(FormData { data: res })
    }
}
