use std::str::from_utf8;
use pnet::packet::udp::UdpPacket;
use crate::mdns::types::{MDNSAnswer, MDNSMessageHeader, MDNSQuestion, MDNSRecordType};

pub fn is_mdns_packet(udp_packet: &UdpPacket) -> bool {
    udp_packet.get_source() == 5353 || udp_packet.get_destination() == 5353
}

pub fn parse_mdns_header(udp_payload: &[u8]) -> Option<MDNSMessageHeader> {
    let query_identifier = ((udp_payload[0] as u16) << 8) | udp_payload[1] as u16;
    let flags = ((udp_payload[2] as u16) << 8) | udp_payload[3] as u16;
    let question_count = ((udp_payload[4] as u16) << 8) | udp_payload[5] as u16;
    let answer_count = ((udp_payload[6] as u16) << 8) | udp_payload[7] as u16;
    let authority_count = ((udp_payload[8] as u16) << 8) | udp_payload[9] as u16;
    let additional_count = ((udp_payload[10] as u16) << 8) | udp_payload[11] as u16;

    let packet = MDNSMessageHeader {
        raw: udp_payload.to_vec(),
        query_identifier: query_identifier,
        flags: flags,
        question_count: question_count,
        answer_count: answer_count,
        authority_count: authority_count,
        additional_count: additional_count
    };
    Some(packet)
}

pub fn parse_mdns_questions(question_count: usize, bytes: &[u8], start_index: usize) -> (usize, Vec<MDNSQuestion>)
{
    let mut questions = Vec::with_capacity(question_count);
    let mut byte_index: usize = start_index;
    for _ in 0..question_count
    {
        let (bytes_read, labels_str) = parse_labels(&bytes, byte_index);
        let labels_raw = &bytes[byte_index..(byte_index + bytes_read)];
        byte_index += bytes_read;
        let question_data = &bytes[byte_index..(byte_index + 4)];
        byte_index += 4;

        let question_type = ((question_data[0] as u16) << 8) | (question_data[1] as u16);
        let question_class = ((question_data[2] as u16) << 8) | (question_data[3] as u16);
        let name = labels_str.join(".");

        questions.push(MDNSQuestion {
            labels_raw: labels_raw.to_vec(),
            labels: labels_str,
            name: name,
            question_type: question_type,
            question_class: question_class
        });
    }
    return (byte_index - 1, questions);
}

pub fn parse_mdns_answers(answer_count: u16, bytes: &[u8], start_byte: usize) -> (usize, Vec<MDNSAnswer>)
{
    let mut answers = Vec::with_capacity(answer_count as usize);
    let mut byte_index: usize = start_byte;
    for i in 0..answer_count
    {
        println!("Parsing answer number {}", i+1);
        let mut current_byte = bytes[byte_index];
        let (bytes_read, labels_str) = parse_labels(bytes, byte_index);
        let labels_raw = &bytes[byte_index..(byte_index + bytes_read)];
        println!("Labels: {:?}, bytes read: {}", labels_str, bytes_read);
        byte_index += bytes_read;
        current_byte = bytes[byte_index];
        let info_bytes = &bytes[byte_index..(byte_index + 10)];
        println!("Info bytes: {:?}", info_bytes);

        let answer_type = ((info_bytes[0] as u16) << 8) | (info_bytes[1] as u16);
        let answer_class = ((info_bytes[2] as u16) << 8) | (info_bytes[3] as u16);
        let ttl: u32 = (((info_bytes[4] as u32) << 24)
            | ((info_bytes[5] as u32) << 16)
            | (info_bytes[6] as u32) << 8)
            | (info_bytes[7] as u32);
        let rd_length: u16 = ((info_bytes[8] as u16) << 8) | (info_bytes[9] as u16);
        println!("current_byte: {}", current_byte);
        println!("rd_length: {}", rd_length);
        let mut rdata_raw: Vec<u8> = Vec::with_capacity(rd_length as usize);

        byte_index += 10;
        for i in 0..rd_length {
            rdata_raw.push(bytes[byte_index + i as usize])
        }

        let record_type = MDNSRecordType::from_u16(answer_type).expect("Invalid answer type.");
        let rdata_labels = match record_type {
            MDNSRecordType::A => None,
            _ => Some(parse_labels(&bytes, byte_index).1)
        };

        answers.push(MDNSAnswer {
            labels_raw: labels_raw.to_vec(),
            labels: labels_str.iter().map(|l| l.to_string()).collect(),
            name: labels_str.join("."),
            answer_type: answer_type,
            answer_class: answer_class,
            ttl: ttl,
            rd_length: rd_length,
            rdata_raw: rdata_raw,
            rdata_labels: rdata_labels
        });
        byte_index += rd_length as usize;
    }
    return (byte_index, answers);
}

fn is_label_pointer(byte: u8) -> bool
{
    return (byte & 0b11000000) == 0b11000000;
}

fn get_pointer(firstByte: u8, secondByte: u8) -> u16
{
    return (((firstByte & 0b00111111) as u16) << 8) | secondByte as u16;
}

fn parse_label(bytes: &[u8], start_index: usize) -> (usize, String)
{
    let mut current_byte = bytes[start_index];
    let length = current_byte as usize;
    let label_start_index = start_index + 1;
    let label_end_index = label_start_index + length;
    let label_raw = &bytes[label_start_index..label_end_index];
    return (length + 1, from_utf8(&label_raw).unwrap().to_string());
}

fn parse_labels(bytes: &[u8], start_index: usize) -> (usize, Vec<String>)
{
    let mut byte_index = start_index;
    let mut labels: Vec<String> = vec![];
    let mut read_bytes_total = 0;
    let mut has_a_pointer = false;
    while bytes.get(byte_index).is_some() && bytes[byte_index] != 0 && !has_a_pointer {
        if is_label_pointer(bytes[byte_index]) {
            has_a_pointer = true;
            let referenced_byte_index = get_pointer(bytes[byte_index], bytes[byte_index + 1]);
            let (_, referenced_labels) = parse_labels(bytes, referenced_byte_index as usize);
            referenced_labels.iter().for_each(|l| labels.push(l.to_owned()));
            byte_index += 2; // Skip the pointer.
            read_bytes_total += 2;
        }
        else {
            let (read_bytes, label) = parse_label(bytes, byte_index);
            labels.push(label);
            byte_index = byte_index + read_bytes;
            read_bytes_total += read_bytes;
        }
    }

    if has_a_pointer {
        (read_bytes_total, labels)
    }
    else{
        // Domain name either ends with a 0 or a pointer. If it does not end with a pointer, read also the 0.
        (read_bytes_total + 1, labels)
    }
}