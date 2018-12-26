pub mod methods;

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

    // Just a syntax sugar
    pub fn command(self, command: &str) -> Sentence {
        self.add(command.to_owned())
    }

    pub fn attribute(self, attr: &str, value: &str) -> Sentence {
        self.add(format!("={}={}", attr, value))
    }

    pub fn query_attribute_exists(self, attr: &str) -> Sentence {
        self.add(format!("?{}", attr))
    }

    pub fn query_no_attribute_exists(self, attr: &str) -> Sentence {
        self.add(format!("?-{}", attr))
    }

    pub fn query_attribute(self, attr: &str, value: &str) -> Sentence {
        self.add(format!("?{}={}", attr, value))
    }

    pub fn query_attribute_greater(self, attr: &str, value: &str) -> Sentence {
        self.add(format!("?>{}={}", attr, value))
    }

    pub fn query_attribute_lesser(self, attr: &str, value: &str) -> Sentence {
        self.add(format!("?<{}={}", attr, value))
    }

    pub fn query_operations(self, ops: &str) -> Sentence {
        self.add(format!("?#{}", ops))
    }

    pub fn add_word(mut self, word: Word) -> Sentence {
        self.words.push(word);
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

use std::net::TcpStream;
pub struct Connection {
    stream: TcpStream,
    // Username and password are stored for the case if router will close the connection, e.g. because of network error
    username: String,
    password: String
}

impl Connection {
    pub fn new(addr: &str, username: &str, password: &str) -> Option<Connection> {
        match TcpStream::connect(addr) {
            Ok(stream) => {
                let mut connection = Connection {
                    stream, username: username.to_owned(), password: password.to_owned()
                };
                methods::login(&mut connection, username.to_owned(), password.to_owned());
                Some(connection)
            },
            Err(_) => None
        }
    }

    fn decode_length(data: &mut [u8]) -> u32 {
        let length: [u8; 4];
        let add: u32;
        if data[0] == 0xE0 {
            length = [data[4], data[3], data[2], data[1]];
            add = 5;
        } else if (data[0] & 0xE0) == 0xE0 {
            data[0] ^= 0xE0;
            length = [data[3], data[2], data[1], data[0]];
            add = 4;
        } else if (data[0] & 0xC0) == 0xC0 {
            data[0] ^= 0xC0;
            length = [data[2], data[1], data[0], 0];
            add = 3;
        } else if (data[0] & 0x80) == 0x80 {
            data[0] ^= 0x80;
            length = [data[1], data[0], 0, 0];
            add = 2;
        } else {
            length = [data[0], 0, 0, 0];
            add = 1;
        }
        unsafe {
            std::mem::transmute::<[u8;4], u32>(length) + add
        }
    }

    // TODO: Handle IO errors properly
    pub fn send_sentence(&mut self, sentence: Sentence) {
        use std::io::Write;
        self.stream.write(&sentence.collect());
    }

    pub fn read_word(&mut self) -> Word {
        use std::io::Read;
        let mut buf = [0; 5];
        self.stream.peek(&mut buf);
        let len = Connection::decode_length(&mut buf);
        let mut buf: Vec<u8> = vec![0; len as usize];
        self.stream.read(buf.as_mut());
        Word::parse(buf.as_ref())
    }
}
