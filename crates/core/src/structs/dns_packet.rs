use super::{BytePacketBuffer as Buffer, DnsHeader, DnsQuestion};
use crate::enums::{BytePacketError as Error, DnsRecord, QueryType};

use std::net::Ipv4Addr;

// * >>> *

#[derive(Debug, Clone)]
pub struct DnsPacket {
  pub header: DnsHeader,
  pub authorities: Vec<DnsRecord>,
  pub questions: Vec<DnsQuestion>,
  pub resources: Vec<DnsRecord>,
  pub answers: Vec<DnsRecord>,
}

impl DnsPacket {
  pub fn new() -> Self {
    Self {
      header: DnsHeader::new(),
      authorities: Vec::new(),
      questions: Vec::new(),
      resources: Vec::new(),
      answers: Vec::new(),
    }
  }

  // *

  pub fn from_buffer(buffer: &mut Buffer) -> Result<Self, Error> {
    let mut packet: Self = Self::new();
    packet.header.read(buffer)?;

    packet.questions = (0..packet.header.questions)
      .map(|_| {
        let mut question: DnsQuestion = DnsQuestion {
          qtype: QueryType::UNKNOWN(0),
          name: String::new(),
        };

        question.read(buffer)?;
        Ok(question)
      })
      .collect::<Result<Vec<DnsQuestion>, Error>>()?;

    packet.answers = (0..packet.header.answers)
      .map(|_| DnsRecord::read(buffer))
      .collect::<Result<_, _>>()?;

    packet.authorities = (0..packet.header.authoritative_entries)
      .map(|_| DnsRecord::read(buffer))
      .collect::<Result<_, _>>()?;

    packet.resources = (0..packet.header.resource_entries)
      .map(|_| DnsRecord::read(buffer))
      .collect::<Result<_, _>>()?;

    Ok(packet)
  }

  // *

  pub fn write(&mut self, buffer: &mut Buffer) -> Result<(), Error> {
    self.header.questions = self.questions.len() as u16;
    self.header.answers = self.answers.len() as u16;
    self.header.authoritative_entries = self.authorities.len() as u16;
    self.header.resource_entries = self.resources.len() as u16;
    self.header.write(buffer)?;

    for question in self.questions.iter_mut() {
      question.write(buffer)?;
    }
    for record in &self.answers {
      record.write(buffer)?;
    }
    for record in &self.authorities {
      record.write(buffer)?;
    }
    for record in &self.resources {
      record.write(buffer)?;
    }

    Ok(())
  }

  // *

  pub fn get_random_a(&self) -> Option<Ipv4Addr> {
    self.answers.iter().find_map(|record| {
      if let DnsRecord::A { address, .. } = record {
        Some(*address)
      } else {
        None
      }
    })
  }

  // *

  pub fn get_ns<'a>(
    &'a self,
    qname: &'a str,
  ) -> impl Iterator<Item = (&'a str, &'a str)> {
    self.authorities.iter().filter_map(move |record| {
      if let DnsRecord::NS { domain, host, .. } = record {
        if qname.ends_with(domain) {
          Some((domain.as_str(), host.as_str()))
        } else {
          None
        }
      } else {
        None
      }
    })
  }

  // *

  pub fn get_resolved_ns(&self, qname: &str) -> Option<Ipv4Addr> {
    self.get_ns(qname).find_map(|(_, host)| {
      self
        .resources
        .iter()
        .filter_map(|record| {
          if let DnsRecord::A {
            address, domain, ..
          } = record
          {
            (domain == host).then_some(*address)
          } else {
            None
          }
        })
        .next()
    })
  }

  // *

  pub fn get_unresolved_ns<'a>(&'a self, qname: &'a str) -> Option<&'a str> {
    self.get_ns(qname).map(|(_, host)| host).next()
  }
}
