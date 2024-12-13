use royal::Message;

fn main() {
    parse_msg();
    //parse_text();
}

fn parse_msg() {
    let messages = Message::parse_msg("test/test_data.msg");

    for message in messages {
        println!("Message ID: {}", message.header.message_id);
        println!("Content: {}", message.content);
    }
}

#[allow(dead_code)]
fn parse_text() {
    let input = "[msg PFM_BTTL_2 [Morgana]][s][f 4 10 65535 0 0][f 5 13 30 15 5]I cant see an exit we're stuck here[f 1 3 65535][w][e]";

    if let Some(message) = Message::parse(input) {
        println!("Message ID: {}", message.header.message_id);
        if let Some(character) = message.header.character {
            println!("Character: {}", character);
        }
        println!("Box Type: {:?}", message.header.box_type);
        println!("Content: {}", message.content);
        println!("Has lipsync: {}", message.flags.has_lipsync);
        println!("Waits for input: {}", message.flags.wait_for_input);
        if let Some(confidant_points) = message.confidant_points {
            println!("Confidant Points: {:?}", confidant_points);
        }
    } else {
        println!("Failed to parse message");
    }
}