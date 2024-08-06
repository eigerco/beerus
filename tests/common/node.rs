use std::collections::HashMap;

use wiremock::{Match, Mock, MockGuard, MockServer, ResponseTemplate};

use super::{
    api::StarknetMatcher,
    matchers::{
        AddDeclareTransactionMatcher, ChainIdMatcher, ClassMatcher,
        EstimateFeeMatcher, NonceMatcher, Response, SpecVersionMatcher,
    },
};

pub struct StarknetNode {
    pub server: MockServer,
    pub mock_guard: Vec<MockGuard>,
}

impl StarknetNode {
    pub async fn new() -> Self {
        let server = MockServer::start().await;
        Self { server, mock_guard: vec![] }
    }

    pub async fn add_methods(
        &mut self,
        requests: HashMap<StarknetMatcher, u64>,
    ) {
        let mut vec_mock_guards = Vec::with_capacity(requests.len());
        for (request, num_request) in requests.into_iter() {
            let mock_guard = match request {
                StarknetMatcher::AddDeclareTransaction => {
                    self.create_mock_guard(
                        AddDeclareTransactionMatcher::default(),
                        num_request,
                    )
                    .await
                }
                StarknetMatcher::AddDeclareTransactionMalicious => {
                    self.create_mock_guard(
                        AddDeclareTransactionMatcher::malicious(),
                        num_request,
                    )
                    .await
                }
                StarknetMatcher::ClassError => {
                    self.create_mock_guard(ClassMatcher::error(), num_request)
                        .await
                }
                StarknetMatcher::ClassSuccess => {
                    self.create_mock_guard(ClassMatcher::success(), num_request)
                        .await
                }
                StarknetMatcher::ClassMalicious => {
                    self.create_mock_guard(
                        ClassMatcher::malicious(),
                        num_request,
                    )
                    .await
                }
                StarknetMatcher::ChainId => {
                    self.create_mock_guard(
                        ChainIdMatcher::default(),
                        num_request,
                    )
                    .await
                }
                StarknetMatcher::ChainIdMalicious => {
                    self.create_mock_guard(
                        ChainIdMatcher::malicious(),
                        num_request,
                    )
                    .await
                }
                StarknetMatcher::EstimateFee => {
                    self.create_mock_guard(
                        EstimateFeeMatcher::default(),
                        num_request,
                    )
                    .await
                }
                StarknetMatcher::EstimateFeeMalicious => {
                    self.create_mock_guard(
                        EstimateFeeMatcher::malicious(),
                        num_request,
                    )
                    .await
                }
                StarknetMatcher::Nonce => {
                    self.create_mock_guard(NonceMatcher::default(), num_request)
                        .await
                }
                StarknetMatcher::NonceMalicious => {
                    self.create_mock_guard(
                        NonceMatcher::malicious(),
                        num_request,
                    )
                    .await
                }
                StarknetMatcher::SpecVersion => {
                    self.create_mock_guard(
                        SpecVersionMatcher::default(),
                        num_request,
                    )
                    .await
                }
                StarknetMatcher::SpecVersionMalicious => {
                    self.create_mock_guard(
                        SpecVersionMatcher::malicious(),
                        num_request,
                    )
                    .await
                }
            };
            vec_mock_guards.push(mock_guard);
        }
        self.mock_guard = vec_mock_guards;
    }

    async fn create_mock_guard(
        &self,
        matcher: impl Match + Response + 'static,
        num_request: u64,
    ) -> MockGuard {
        let response = matcher.response();
        Mock::given(matcher)
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .expect(num_request)
            .mount_as_scoped(&self.server)
            .await
    }
}
