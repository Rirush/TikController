use super::{Connection, Sentence};

pub fn login(c: &mut Connection, username: String, password: String) {
    c.send_sentence(Sentence::new().command("/login").attribute("name", &username).attribute("password", &password).add("".to_string()));
}
