use crate::{blob::responses::PutBlockResponse, prelude::*};
use azure_core::prelude::*;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct AppendBlockBuilder<'a> {
    blob_client: &'a BlobClient,
    body: Bytes,
    hash: Option<&'a Hash>,
    condition_max_size: Option<ConditionMaxSize>,
    condition_append_position: Option<ConditionAppendPosition>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> AppendBlockBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient, body: impl Into<Bytes>) -> Self {
        Self {
            blob_client,
            body: body.into(),
            hash: None,
            condition_max_size: None,
            condition_append_position: None,
            lease_id: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        hash: &'a Hash => Some(hash),
        condition_max_size: ConditionMaxSize => Some(condition_max_size),
        condition_append_position: ConditionAppendPosition => Some(condition_append_position),
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(&self) -> azure_core::Result<PutBlockResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);
        url.query_pairs_mut().append_pair("comp", "appendblock");

        let mut request = self.blob_client.prepare_request(
            url.as_str(),
            http::Method::PUT,
            Some(self.body.clone()),
        )?;
        request.add_optional_header_ref(&self.hash);
        request.add_optional_header(&self.condition_max_size);
        request.add_optional_header(&self.condition_append_position);
        request.add_optional_header_ref(&self.lease_id);
        request.add_optional_header(&self.client_request_id);

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        PutBlockResponse::from_headers(response.headers())
    }
}
