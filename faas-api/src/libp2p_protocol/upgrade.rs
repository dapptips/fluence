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

use crate::{FunctionCall, ProtocolMessage};
pub use failure::Error;
use futures::{AsyncRead, AsyncWrite, AsyncWriteExt, Future};
use libp2p::core::{upgrade, InboundUpgrade, OutboundUpgrade, UpgradeInfo};
use std::{io, iter, pin::Pin};

// 1 Mb
#[allow(clippy::identity_op)]
const MAX_BUF_SIZE: usize = 1 * 1024 * 1024;
const PROTOCOL_INFO: &[u8] = b"/fluence/faas/1.0.0";

impl UpgradeInfo for ProtocolMessage {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(PROTOCOL_INFO)
    }
}

impl<Socket> InboundUpgrade<Socket> for ProtocolMessage
where
    Socket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = ProtocolMessage;
    type Error = Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_inbound(self, mut socket: Socket, info: Self::Info) -> Self::Future {
        Box::pin(async move {
            let packet = upgrade::read_one(&mut socket, MAX_BUF_SIZE).await?;
            // TODO: remove that once debugged
            match std::str::from_utf8(&packet) {
                Ok(str) => log::debug!("Got inbound ProtocolMessage: {}", str),
                Err(err) => log::warn!("Can't parse inbound ProtocolMessage to UTF8 {}", err),
            }

            match serde_json::from_slice(&packet) {
                Ok(message) => {
                    socket.close().await?;
                    Ok(message)
                }
                Err(err) => {
                    // Generate and send error back through socket
                    let err_msg = gen_error(&err, &packet);
                    err_msg.upgrade_outbound(socket, info).await?;
                    return Err(err.into());
                }
            }
        })
    }
}

fn gen_error<E: std::error::Error>(err: &E, data: &[u8]) -> ProtocolMessage {
    use serde_json::json;
    ProtocolMessage::FunctionCall(FunctionCall {
        uuid: "error".into(),
        target: None,
        reply_to: None,
        arguments: json!({ "data": data }),
        name: Some(err.to_string()),
    })
}

impl<Socket> OutboundUpgrade<Socket> for ProtocolMessage
where
    Socket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = ();
    type Error = io::Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_outbound(self, mut socket: Socket, _: Self::Info) -> Self::Future {
        Box::pin(async move {
            // TODO: remove that once debugged
            match serde_json::to_string(&self) {
                Ok(str) => log::debug!("Sending outbound ProtocolMessage: {}", str),
                Err(err) => log::warn!("Can't serialize {:?} to string {}", &self, err),
            }

            let bytes = serde_json::to_vec(&self)?;
            upgrade::write_one(&mut socket, bytes).await?;

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ProtocolMessage;
    use crate::call_test_utils::gen_ipfs_call;
    use futures::prelude::*;
    use libp2p::core::{
        multiaddr::multiaddr,
        transport::{memory::MemoryTransport, ListenerEvent, Transport},
        upgrade,
    };

    use rand::{thread_rng, Rng};

    #[test]
    fn oneshot_channel_test() {
        let mem_addr = multiaddr![Memory(thread_rng().gen::<u64>())];
        let mut listener = MemoryTransport.listen_on(mem_addr).unwrap();
        let listener_addr =
            if let Some(Some(Ok(ListenerEvent::NewAddress(a)))) = listener.next().now_or_never() {
                a
            } else {
                panic!("MemoryTransport not listening on an address!");
            };

        let inbound = async_std::task::spawn(async move {
            let listener_event = listener.next().await.unwrap();
            let (listener_upgrade, _) = listener_event.unwrap().into_upgrade().unwrap();
            let conn = listener_upgrade.await.unwrap();
            let upgrade = ProtocolMessage::Upgrade;
            upgrade::apply_inbound(conn, upgrade).await.unwrap()
        });

        let sent_call = async_std::task::block_on(async move {
            let call = ProtocolMessage::FunctionCall(gen_ipfs_call());
            let c = MemoryTransport.dial(listener_addr).unwrap().await.unwrap();
            upgrade::apply_outbound(c, call.clone(), upgrade::Version::V1)
                .await
                .unwrap();
            call
        });

        let received_call = futures::executor::block_on(inbound);

        assert_eq!(sent_call, received_call);
    }
}
