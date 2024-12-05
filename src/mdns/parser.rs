use std::str::from_utf8;
use pnet::packet::udp::UdpPacket;
use crate::mdns::types::{MDNSAnswer, MDNSMessageHeader, MDNSQuestion};

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
    for i in 0..question_count
    {
        let mut current_byte = bytes[byte_index];
        let (bytes_read, labels_str) = parse_labels(&bytes, byte_index);
        let labels_raw = &bytes[byte_index..(byte_index + bytes_read)];
        byte_index += bytes_read + 1;
        current_byte = bytes[byte_index];

        let question_type = ((current_byte as u16) << 8) | (bytes[byte_index + 1] as u16);
        let question_class = ((bytes[byte_index + 2] as u16) << 8) | (bytes[byte_index + 3] as u16);

        questions.push(MDNSQuestion {
            labels_raw: labels_raw.to_vec(),
            labels: labels_str,
            question_type: question_type,
            question_class: question_class
        });
        byte_index += 4;
    }
    return (byte_index - 1, questions);
}

pub fn parse_mdns_answers(answer_count: u16, bytes: &[u8], start_byte: usize) -> (usize, Vec<MDNSAnswer>)
{
    let mut answers = Vec::with_capacity(answer_count as usize);
    let mut byte_index: usize = start_byte;
    for i in 0..answer_count
    {
        let mut current_byte = bytes[byte_index];
        let (bytes_read, labels_str) = parse_labels(bytes, 0);
        let labels_raw = &bytes[byte_index..(bytes_read + 1)];
        byte_index += bytes_read + 1;
        current_byte = bytes[byte_index];

        let answer_type = ((current_byte as u16) << 8) | (bytes[byte_index + 1] as u16);
        let answer_class = ((bytes[byte_index + 2] as u16) << 8) | (bytes[byte_index + 3] as u16);
        let ttl: u32 = (((bytes[byte_index + 4] as u32) << 24)
            | ((bytes[byte_index + 5] as u32) << 16)
            | (bytes[byte_index + 6] as u32) << 8)
            | (bytes[byte_index + 7] as u32);
        let rd_length: u16 = ((bytes[byte_index + 8] as u16) << 8) | (bytes[byte_index + 9] as u16);
        let mut rdata_raw: Vec<u8> = Vec::with_capacity(rd_length as usize);

        byte_index += 10;
        for i in 0..rd_length {
            rdata_raw.push(bytes[byte_index + i as usize])
        }
        let rdata_labels = parse_labels(&bytes, byte_index);

        answers.push(MDNSAnswer {
            labels_raw: labels_raw.to_vec(),
            labels: labels_str.iter().map(|l| l.to_string()).collect(),
            answer_type: answer_type,
            answer_class: answer_class,
            ttl: ttl,
            rd_length: rd_length,
            rdata_raw: rdata_raw,
            rdata_labels: Some(rdata_labels.1)
        });
        byte_index += (rd_length + 1) as usize;
    }
    return (byte_index - 1, answers);
}

fn parse_label(bytes: &[u8], start_index: usize) -> (usize, String)
{
    let mut current_byte = bytes[start_index];
    if (current_byte & 0b11000000) == 0b11000000 {
        let pointer = (((current_byte & 0b00111111) as u16) << 8) | bytes[start_index + 1] as u16;
        return (2, parse_label(bytes, pointer as usize).1);
    } else {
        let length = current_byte as usize;
        let label_start_index = start_index + 1;
        let label_end_index = label_start_index + length;
        let label_raw = &bytes[label_start_index..label_end_index];
        return (length + 1, from_utf8(&label_raw).unwrap().to_string());
    }
}

fn parse_labels(bytes: &[u8], start_index: usize) -> (usize, Vec<String>)
{
    let mut byte_index = start_index;
    let mut current_byte = Some(bytes[byte_index].clone());
    let mut labels: Vec<String> = vec![];
    let mut read_bytes_total = 0;
    while current_byte.is_some() && current_byte.unwrap() != 0 {
        let (read_bytes, label) = parse_label(bytes, byte_index);
        labels.push(label);
        byte_index = byte_index + read_bytes;
        current_byte = bytes.get(byte_index).map(|x| x.clone());
        read_bytes_total += read_bytes;
    }
    return (read_bytes_total, labels);
}