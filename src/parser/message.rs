use std::{fs::File, io::BufReader, io::BufRead};

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
    pub confidant_points: Option<ConfidantPoints>,
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

#[derive(Debug)]
pub struct ConfidantPoints {
    pub confidant_id: u8,
    pub points: u8,
    pub model_id: u16,
}

impl Message {
    pub fn parse(input: &str) -> Option<Message> {
        let mut parts = input.splitn(2, ']');
        let header_part = parts.next()?;
        let content_part = parts.next()?;

        let header = Self::parse_header(header_part)?;
        
        if !content_part.contains("[s]") {
            println!("Message missing [s] tag: {}", input);
            return None;
        }

        let (content, flags, confidant_points) = Self::parse_content(content_part);

        Some(Message {
            header,
            content,
            flags,
            confidant_points,
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

    fn parse_content(content: &str) -> (String, MessageFlags, Option<ConfidantPoints>) {
        let mut flags = MessageFlags {
            has_lipsync: false,
            wait_for_input: false,
        };

        let mut message = String::new();
        let mut confidant_points: Option<ConfidantPoints> = None;
        let mut parts = content.split('[').collect::<Vec<_>>();

        for (i, part) in parts.iter().enumerate() {
            if part.is_empty() {
                continue;
            }

            if part.starts_with('s') {
                if let Some(text) = part.split(']').nth(1) {
                    if !text.is_empty() {
                        message.push_str(text);
                    }
                }
            } else if part.starts_with("f 4 10") {
                flags.has_lipsync = true;
                if let Some(text) = part.split(']').nth(1) {
                    message.push_str(text);
                }
            } else if part.starts_with('w') {
                flags.wait_for_input = true;
            } else if part.starts_with('e') {
                break;
            } else if part.starts_with("f 5 13 ") {
                confidant_points = {
                    let confidant_part = part.split(']').next().unwrap_or("");
                    let parts: Vec<&str> = confidant_part["f 5 13 ".len()..].split_whitespace().collect();
                    if parts.len() >= 3 {
                        let model_id = parts[2].trim_end_matches(']');
                        if let (Ok(confidant_id), Ok(points), Ok(model_id)) = (
                            parts[0].parse::<u32>(),
                            parts[1].parse::<u32>(),
                            model_id.parse::<u32>()
                        ) {
                            Some(ConfidantPoints {
                                confidant_id: confidant_id as u8,
                                points: points as u8,
                                model_id: model_id as u16,
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };
                if let Some(text) = part.split(']').nth(1) {
                    message.push_str(text);
                }
            } else if part.starts_with('f') {
                if let Some(text) = part.split(']').nth(1) {
                    message.push_str(text);
                }
            } else {
                let text = if part.contains(']') {
                    part.split(']').nth(1).unwrap_or(part)
                } else {
                    part
                };
                if !text.is_empty() {
                    message.push_str(text);
                }
            }
        }

        (message.trim().to_string(), flags, confidant_points)
    }

    pub fn parse_msg(file_path: &str) -> Vec<Message> {
        let mut messages = Vec::new();
        let mut current_message = String::new();
        
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);
            
            for line in reader.lines() {
                if let Ok(line) = line {
                    let trimmed = line.trim();
                    
                    if trimmed.is_empty() || trimmed.starts_with("//") {
                        continue;
                    }
                    
                    if trimmed.starts_with("[msg") {
                        if !current_message.is_empty() {
                            if let Some(message) = Self::parse(&current_message) {
                                messages.push(message);
                            }
                            current_message.clear();
                        }
                        current_message.push_str(trimmed);
                    } else {
                        current_message.push_str(trimmed);
                    }
                }
            }
            
            if !current_message.is_empty() {
                if let Some(message) = Self::parse(&current_message) {
                    messages.push(message);
                }
            }
        } else {
            println!("Failed to open file: {}", file_path);
        }
        
        messages
    }
}
