use std::fmt::{Display, Formatter};

pub struct MDNSQuestion{
    pub labels_raw: Vec<u8>,
    pub labels: Vec<String>,
    pub question_type: u16, // first byte is terminator for labels and as such is always 0
    pub question_class: u16
}

pub struct MDNSAnswer{
    pub labels_raw: Vec<u8>,
    pub labels: Vec<String>,
    pub answer_type: u16, // first byte is terminator for labels and as such is always 0
    pub answer_class: u16,
    pub ttl: u32,
    pub rd_length: u16,
    pub rdata_raw: Vec<u8>,
    pub rdata_labels: Option<Vec<String>> // TODO: This is only valid on certain answer types.
}

pub struct MDNSMessageHeader {
    pub raw: Vec<u8>,
    pub query_identifier: u16,
    pub flags: u16,
    pub question_count: u16,
    pub answer_count: u16,
    pub authority_count: u16,
    pub additional_count: u16
}

impl Display for MDNSMessageHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, flags: {}, question_count: {}, answer_count: {}, authority_count: {}, additional_count: {}",
               self.query_identifier,
               self.flags,
               self.question_count,
               self.answer_count,
               self.authority_count,
               self.additional_count
        )
    }
}

impl MDNSMessageHeader {
}