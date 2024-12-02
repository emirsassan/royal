#[derive(Debug)]
pub struct MessageHeader {
    pub box_type: BoxType,
    pub message_id: String,
    pub character: Option<String>,
}

#[derive(Debug)]
pub struct MessageFlags {
    pub has_lipsync: bool,
    pub wait_for_input: bool,
}

#[derive(Debug)]
pub struct Message {
    pub header: MessageHeader,
    pub content: String,
    pub flags: MessageFlags,
}

#[derive(Debug)]
pub enum BoxType {
    Help,
    Message,
    Mind,
    System,
    Trivia,
    Devil,
    Progress,
    Unknown,
}

impl Message {
    pub fn parse(input: &str) -> Option<Message> {
        let parts: Vec<&str> = input.split(']').collect();
        if parts.len() < 2 {
            return None;
        }

        let header = Self::parse_header(parts[0])?;

        let content_part = &parts[1..].join("]");
        let (content, flags) = Self::parse_content(content_part);

        Some(Message {
            header,
            content,
            flags,
        })
    }

    fn parse_header(header: &str) -> Option<MessageHeader> {
        let header = header.trim_start_matches('[');
        let parts: Vec<&str> = header.split_whitespace().collect();

        if parts.len() < 3 {
            return None;
        }

        if parts[0] != "msg" {
            return None;
        }

        let character = if parts[2].starts_with('[') {
            Some(parts[2].trim_matches(|c| c == '[' || c == ']').to_string())
        } else {
            None
        };
        let box_type = match &parts[1][..3] {
            "HLP" => BoxType::Help,
            "MSG" => BoxType::Message,
            "MND" => BoxType::Mind,
            "SYS" => BoxType::System,
            "TRV" => BoxType::Trivia,
            "DVL" => BoxType::Devil,
            "PFM" => BoxType::Progress,
            _ => BoxType::Unknown,
        };
        let message_id = parts[1].to_string();

        Some(MessageHeader {
            box_type,
            message_id,
            character,
        })
    }

    fn parse_content(content: &str) -> (String, MessageFlags) {
        let mut flags = MessageFlags {
            has_lipsync: false,
            wait_for_input: false,
        };

        let mut message = String::new();
        let mut parts = content.split('[');

        if let Some(first) = parts.next() {
            if first.starts_with(']') {
                // skip leading ]
            }
        }

        for part in parts {
            if part.is_empty() {
                continue;
            }

            if part.starts_with('s') {
                continue;
            } else if part.starts_with("f 4 10") {
                flags.has_lipsync = true;
                if let Some(text) = part.split(']').nth(1) {
                    message.push_str(text);
                }
            } else if part.starts_with('w') {
                flags.wait_for_input = true;
            } else if part.starts_with('e') {
                break;
            } else if !part.starts_with('f') {
                message.push_str(part);
            }
        }

        (message.trim().to_string(), flags)
    }
}
