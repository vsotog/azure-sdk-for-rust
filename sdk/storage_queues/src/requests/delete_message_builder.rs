use crate::clients::PopReceiptClient;
use crate::responses::*;
use azure_core::prelude::*;

use std::convert::TryInto;

#[derive(Debug)]
pub struct DeleteMessageBuilder<'a> {
    pop_receipt_client: &'a PopReceiptClient,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> DeleteMessageBuilder<'a> {
    pub(crate) fn new(pop_receipt_client: &'a PopReceiptClient) -> Self {
        DeleteMessageBuilder {
            pop_receipt_client,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(&self) -> azure_core::Result<DeleteMessageResponse> {
        let mut url = self.pop_receipt_client.pop_receipt_url()?;

        self.timeout.append_to_url_query(&mut url);

        let mut request = self.pop_receipt_client.storage_client().prepare_request(
            url.as_str(),
            http::method::Method::DELETE,
            None,
        )?;
        request.add_optional_header(&self.client_request_id);

        let response = self
            .pop_receipt_client
            .storage_client()
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }
}
