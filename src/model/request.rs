// Copyright 2022 CeresDB Project Authors. Licensed under Apache-2.0.

use ceresdbproto::storage::QueryRequest as QueryRequestPb;

/// Query request
/// Avoid exposed interfaces explicitly depending on ceresproto
#[derive(Debug, Clone)]
pub struct QueryRequest {
    pub metrics: Vec<String>,
    pub ql: String,
}

impl From<QueryRequest> for QueryRequestPb {
    fn from(req: QueryRequest) -> Self {
        let mut pb_req = QueryRequestPb::default();
        pb_req.metrics = req.metrics.into();
        pb_req.ql = req.ql;

        pb_req
    }
}
