use crate::jd_client::error::Error;
use codec_sv2::{StandardEitherFrame, StandardSv2Frame};
use roles_logic_sv2::{
    common_messages_sv2::{Protocol, SetupConnection},
    handlers::common::{ParseUpstreamCommonMessages, SendTo},
    parsers::PoolMessages,
    routing_logic::{CommonRoutingLogic, NoRouting},
    utils::Mutex,
};
use std::{convert::TryInto, net::SocketAddr, sync::Arc};
use tokio::sync::mpsc::{Receiver as TReceiver, Sender as TSender};
use tracing::error;
pub type Message = PoolMessages<'static>;
pub type StdFrame = StandardSv2Frame<Message>;
pub type EitherFrame = StandardEitherFrame<Message>;
pub struct SetupConnectionHandler {}

impl SetupConnectionHandler {
    fn get_setup_connection_message(address: SocketAddr) -> SetupConnection<'static> {
        let endpoint_host = address.ip().to_string().into_bytes().try_into().expect("Internal error: this operation can not fail because IP address can always be converted into Inner");
        let vendor = String::new().try_into().expect("Internal error: this operation can not fail empty String can always be converted into Inner");
        let hardware_version = String::new().try_into().expect("Internal error: this operation can not fail empty String can always be converted into Inner");
        let firmware = String::new().try_into().expect("Internal error: this operation can not fail empty String can always be converted into Inner");
        let device_id = String::new().try_into().expect("Internal error: this operation can not fail empty String can always be converted into Inner");
        SetupConnection {
            protocol: Protocol::TemplateDistributionProtocol,
            min_version: 2,
            max_version: 2,
            flags: 0b0000_0000_0000_0000_0000_0000_0000_0000,
            endpoint_host,
            endpoint_port: address.port(),
            vendor,
            hardware_version,
            firmware,
            device_id,
        }
    }

    pub async fn setup(
        receiver: &mut TReceiver<EitherFrame>,
        sender: &mut TSender<EitherFrame>,
        address: SocketAddr,
    ) -> Result<(), Error> {
        let setup_connection = Self::get_setup_connection_message(address);

        let sv2_frame: StdFrame = PoolMessages::Common(setup_connection.into())
            .try_into()
            .expect("Internal error: this operation can not fail because PoolMessage::Common can always be converted into StdFrame");
        let sv2_frame = sv2_frame.into();
        sender
            .send(sv2_frame)
            .await
            .map_err(|_| Error::Unrecoverable)?;

        let mut incoming: StdFrame = match receiver.recv().await {
            Some(msg) => msg.try_into()?,
            None => {
                error!("Failed to parse incoming SetupConnectionResponse");
                return Err(Error::Unrecoverable);
            }
        };

        let message_type = match incoming.get_header() {
            Some(header) => header.msg_type(),
            None => {
                error!("Message header is None");
                return Err(Error::Unrecoverable);
            }
        };
        let payload = incoming.payload();
        ParseUpstreamCommonMessages::handle_message_common(
            Arc::new(Mutex::new(SetupConnectionHandler {})),
            message_type,
            payload,
            CommonRoutingLogic::None,
        )
        .map_err(Error::RolesSv2Logic)?;
        Ok(())
    }
}

impl ParseUpstreamCommonMessages<NoRouting> for SetupConnectionHandler {
    fn handle_setup_connection_success(
        &mut self,
        _: roles_logic_sv2::common_messages_sv2::SetupConnectionSuccess,
    ) -> Result<roles_logic_sv2::handlers::common::SendTo, roles_logic_sv2::errors::Error> {
        Ok(SendTo::None(None))
    }

    fn handle_setup_connection_error(
        &mut self,
        _: roles_logic_sv2::common_messages_sv2::SetupConnectionError,
    ) -> Result<roles_logic_sv2::handlers::common::SendTo, roles_logic_sv2::errors::Error> {
        todo!()
    }

    fn handle_channel_endpoint_changed(
        &mut self,
        _: roles_logic_sv2::common_messages_sv2::ChannelEndpointChanged,
    ) -> Result<roles_logic_sv2::handlers::common::SendTo, roles_logic_sv2::errors::Error> {
        todo!()
    }
}
