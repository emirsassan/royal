use royal::Message;

fn main() {
    let input = "[msg HLP_BTTL_2 [Morgan]][s][f 4 10 65535 0 0]I cant see an exit we're stuck here[f 1 3 65535][w][e]";

    if let Some(message) = Message::parse(input) {
        println!("Message ID: {}", message.header.message_id);
        if let Some(character) = message.header.character {
            println!("Character: {}", character);
        }
        println!("Box Type: {}", message.header.box_type);
        println!("Content: {}", message.content);
        println!("Has lipsync: {}", message.flags.has_lipsync);
        println!("Waits for input: {}", message.flags.wait_for_input);
    } else {
        println!("Failed to parse message");
    }
}
