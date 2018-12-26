pub struct Word {
    length: Vec<u8>,
    word: String,
}

impl Word {
    fn new(word: String) -> Word {
        let len = word.len();
        match len {
            0...0x7F => Word {
                length: vec![len as u8],
                word,
            },
            0x80...0x3FFF => Word {
                length: vec![(len >> 8 | 0x80) as u8, len as u8],
                word,
            },
            0x40000...0x1FFFFF => Word {
                length: vec![(len >> 16 | 0xC0) as u8, (len >> 8) as u8, len as u8],
                word,
            },
            0x200000...0xFFFFFFF => Word {
                length: vec![
                    (len >> 24 | 0xE0) as u8,
                    (len >> 16) as u8,
                    (len >> 8) as u8,
                    len as u8,
                ],
                word,
            },
            _ => Word {
                length: vec![
                    0xE0,
                    (len >> 24) as u8,
                    (len >> 16) as u8,
                    (len >> 8) as u8,
                    len as u8,
                ],
                word,
            },
        }
    }
    pub fn len(&self) -> usize {
        match self.length.len() {
            1 => self.length[0] as usize,
            2 => ((self.length[0] as usize ^ 0x80) << 8) | self.length[1] as usize,
            3 => {
                (((self.length[0] as usize ^ 0xC0) << 16) | (self.length[1] as usize) << 8)
                    | self.length[2] as usize
            }
            4 => {
                ((((self.length[0] as usize ^ 0xE0) << 24) | (self.length[1] as usize) << 16)
                    | (self.length[2] as usize) << 8)
                    | self.length[3] as usize
            }
            5 => {
                ((((self.length[1] as usize) << 24) | (self.length[2] as usize) << 16)
                    | (self.length[3] as usize) << 8)
                    | self.length[4] as usize
            }
            _ => 0,
        }
    }

    pub fn word(&self) -> &String {
        &self.word
    }

    pub fn parse(data: &[u8]) -> Word {
        let mut word = Word {
            length: vec![],
            word: String::new(),
        };
        let end: usize;
        if data[0] == 0xE0 {
            word.length = data[1..5].to_vec();
            end = 5;
        } else if data[0] & 0xE0 == 0xE0 {
            word.length = data[0..4].to_vec();
            end = 4;
        } else if data[0] & 0xC0 == 0xC0 {
            word.length = data[0..3].to_vec();
            end = 3;
        } else if data[0] & 0x80 == 0x80 {
            word.length = data[0..2].to_vec();
            end = 2;
        } else {
            word.length = vec![data[0]];
            end = 1;
        }
        word.word = String::from_utf8(data[end..end + word.len()].to_vec()).unwrap();
        word
    }
}

pub struct Sentence {
    words: Vec<Word>,
}

impl Sentence {
    pub fn new() -> Sentence {
        Sentence { words: vec![] }
    }

    pub fn add(mut self, word: String) -> Sentence {
        self.words.push(Word::new(word));
        self
    }

    pub fn collect(self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        for mut word in self.words {
            result.append(&mut word.length);
            result.append(&mut word.word.into_bytes());
        }
        result
    }
}
