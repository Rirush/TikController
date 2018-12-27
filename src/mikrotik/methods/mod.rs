use super::{Connection, Sentence};

pub fn login(c: &mut Connection, username: String, password: String) -> Result<(), ()> {
    c.send_sentence(
        Sentence::new()
            .command("/login")
            .attribute("name", &username)
            .attribute("password", &password)
            .add("".to_string()),
    );
    let sentence = c.read_sentence();
    if sentence[0].word() == "!done" {
        Ok(())
    } else {
        c.read_sentence();
        Err(())
    }
}
