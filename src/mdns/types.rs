use std::fmt::{write, Display, Formatter};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MDNSRecordType {
    // Question and Answer types
    A = 1,       // Address record (IPv4)
    NS = 2,      // Name server
    CNAME = 5,   // Canonical name
    SOA = 6,     // Start of authority
    PTR = 12,    // Pointer record
    MX = 15,     // Mail exchange
    TXT = 16,    // Text record
    AAAA = 28,   // Address record (IPv6)
    SRV = 33,    // Service record
    NSEC = 47,   // Next secure record
    OPT = 41,    // Option (used in EDNS)
    ANY = 255,   // Wildcard match (any record type)

    // Uncommon or reserved types
    // HINFO = 13,  // Host information
    // MINFO = 14,  // Mailbox or mailing list information
    AXFR = 252,  // Request for zone transfer
    MAILB = 253, // Request for mailbox-related records
    MAILA = 254, // Request for mail agent records
}

impl Display for MDNSRecordType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let record_type = match self {
            MDNSRecordType::A => "A",
            MDNSRecordType::NS => "NS",
            MDNSRecordType::CNAME => "CNAME",
            MDNSRecordType::SOA => "SOA",
            MDNSRecordType::PTR => "PTR",
            MDNSRecordType::MX => "MX",
            MDNSRecordType::TXT => "TXT",
            MDNSRecordType::AAAA => "AAAA",
            MDNSRecordType::SRV => "SRV",
            MDNSRecordType::NSEC => "NSEC",
            MDNSRecordType::OPT => "OPT",
            MDNSRecordType::ANY => "ANY",
            MDNSRecordType::AXFR => "AXFR",
            MDNSRecordType::MAILB => "MAILB",
            MDNSRecordType::MAILA => "MAILA"
        };
        write!(f, "{}", record_type)
    }
}

#[derive(Clone)]
pub enum MDNSRData {
    A { ipv4_address: Ipv4Addr }, // Maps a hostname to an IPv4 address
    AAAA { ipv6_addr: Ipv6Addr }, // Maps a hostname to an IPv6 address
    PTR { domain_name: String }, // Service discovery
    SRV { priority: u16, weight: u16, port: u16, target_domain_name: String }, // Service instance details
    TXT { text: String }, // Service metadata TODO: This is often key-value pair
    CNAME { canonical_domain_name: String }, // Alias for a domain name
    NSEC { raw: Vec<u8> }, // Next domain name, Bitmap indicating available record types TODO: implement strongly typed NSEC data
    ANY { raw: Vec<u8> }, // No specific RDATA; used in queries.
    OTHER { raw: Vec<u8> } // Others
}

impl Display for MDNSRData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MDNSRData::A { ipv4_address } => { write!(f, "A ({})", ipv4_address) }
            MDNSRData::AAAA { ipv6_addr } => { write!(f, "AAAA ({})", ipv6_addr) }
            MDNSRData::PTR { domain_name } => { write!(f, "PTR ({})", domain_name) }
            MDNSRData::SRV { priority, weight, port, target_domain_name } => { write!(f, "SRV ({}, {}, {}, {})", priority, weight, port, target_domain_name) }
            MDNSRData::TXT { text } => { write!(f, "TXT ({})", text) }
            MDNSRData::CNAME { canonical_domain_name } => { write!(f, "CNAME ({})", canonical_domain_name) }
            MDNSRData::NSEC { .. } => { write!(f, "NSEC") }
            MDNSRData::ANY { .. } => { write!(f, "ANY") }
            MDNSRData::OTHER { .. } => { write!(f, "OTHER") }
        }
    }
}

impl MDNSRecordType {
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            1 => Some(Self::A),
            2 => Some(Self::NS),
            5 => Some(Self::CNAME),
            6 => Some(Self::SOA),
            12 => Some(Self::PTR),
            15 => Some(Self::MX),
            16 => Some(Self::TXT),
            28 => Some(Self::AAAA),
            33 => Some(Self::SRV),
            41 => Some(Self::OPT),
            47 => Some(Self::NSEC),
            252 => Some(Self::AXFR),
            253 => Some(Self::MAILB),
            254 => Some(Self::MAILA),
            255 => Some(Self::ANY),
            _ => None,
        }
    }
}

/// Enum representing the DNS Query Class (QClass)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MDNSQueryClass {
    /// Internet (most common for mDNS)
    IN = 1,

    /// Any class (used in wildcard queries)
    ANY = 255,
}

impl MDNSQueryClass {
    /// Converts a u16 value to an MdnsQueryClass, if possible
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            1 => Some(Self::IN),
            255 => Some(Self::ANY),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct MDNSQuestion{
    pub name: String,
    pub question_type: MDNSRecordType,
    pub question_class: MDNSQueryClass
}

#[derive(Clone)]
pub struct MDNSAnswer{
    pub name: String,
    pub answer_type: MDNSRecordType,
    pub answer_class: MDNSQueryClass,
    pub ttl_seconds: u32,
    pub rd_length: u16,
    pub rdata: MDNSRData
}

#[derive(Clone)]
pub struct MDNSMessageHeader {
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