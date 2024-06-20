

#[derive(Debug, PartialEq)]
pub enum TcpFlags {
    SYN = 0x002,  
}

pub struct Packet {
    
    tcp_flags: u16,
    is_tcp: bool,
}

impl Packet {
    
    pub fn is_tcp(&self) -> bool {
        self.is_tcp
    }

    pub fn tcp_flags(&self) -> u16 {
        self.tcp_flags
    }
}

pub fn analyze_packets(packet: &Packet) -> Option<String> {
    if packet.is_tcp() && packet.tcp_flags() == TcpFlags::SYN as u16 {
        Some("Possible SYN flood attack detected".to_string())
    } else {
        None
    }
}

pub fn start_analysis(data: &str) -> String {
    
    format!("Analysis result for data: {}", data)
}
