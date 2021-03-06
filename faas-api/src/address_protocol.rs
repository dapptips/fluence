/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::address::AddressError;
use libp2p::PeerId;
use std::borrow::Cow;
use std::str::FromStr;

type Result<T> = core::result::Result<T, AddressError>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Protocol {
    // Service on the network
    Service(String),
    // Directly accessible peer
    Peer(PeerId),
    // Peer that's accessible only via relay mechanics
    Client(PeerId),
    Signature(Vec<u8>),
}

impl Protocol {
    #[allow(clippy::should_implement_trait)]
    // Builds Protocol from next 2 elements of the given iterator
    pub fn from_iter<'a, I>(mut iter: I) -> Result<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        use self::Protocol::*;
        match iter.next().ok_or(AddressError::Empty)? {
            "service" => {
                let id = iter.next().ok_or(AddressError::Empty)?;
                Ok(Service(id.into()))
            }
            "peer" => {
                let id = Self::parse_peer_id(iter)?;
                Ok(Peer(id))
            }
            "client" => {
                let id = Self::parse_peer_id(iter)?;
                Ok(Client(id))
            }
            "signature" => {
                let sig = iter.next().ok_or(AddressError::Empty)?;
                let sig = Self::parse_base_58(sig)?;
                Ok(Signature(sig))
            }
            _ => Err(AddressError::UnknownProtocol),
        }
    }

    // Returns tuple of (kind, value).
    // Basically destructs Protocol into a tuple (without consuming it)
    pub fn components(&self) -> (&'static str, Cow<'_, String>) {
        use self::Protocol::*;
        match self {
            Service(id) => ("service", Cow::Borrowed(id)),
            Peer(id) => ("peer", Cow::Owned(Self::peer_id_to_base58(id))),
            Client(id) => ("client", Cow::Owned(Self::peer_id_to_base58(id))),
            Signature(sig) => ("signature", Cow::Owned(Self::vec_to_base58(sig))),
        }
    }

    // Utility function to parse peer id from iterator. Takes two elements from the iterator.
    fn parse_peer_id<'a, I>(mut iter: I) -> Result<PeerId>
    where
        I: Iterator<Item = &'a str>,
    {
        let str = iter.next().ok_or(AddressError::Empty)?;
        let peer_id: PeerId = str.parse().map_err(|_| AddressError::InvalidPeerId)?;
        Ok(peer_id)
    }

    fn parse_base_58(from: &str) -> Result<Vec<u8>> {
        bs58::decode(from)
            .into_vec()
            .map_err(|_| AddressError::InvalidProtocol)
    }

    fn peer_id_to_base58(peer_id: &PeerId) -> String {
        bs58::encode(peer_id.as_bytes()).into_string()
    }

    fn vec_to_base58(v: &[u8]) -> String {
        bs58::encode(v).into_string()
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (proto, value) = self.components();
        write!(f, "/{}/{}", proto, value)
    }
}

impl FromStr for Protocol {
    type Err = AddressError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut parts = s.split('/').peekable();

        if Some("") != parts.next() {
            // Protocol must start with `/`
            return Err(AddressError::InvalidProtocol);
        }

        Protocol::from_iter(parts)
    }
}
