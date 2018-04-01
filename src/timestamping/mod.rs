// Copyright 2018 Ivan Penkin <grek.penkin@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


pub mod schema;
pub mod transactions;
mod errors;
mod contracts;
mod api;

use exonum::blockchain::{Service, Transaction, ApiContext, TransactionSet};
use exonum::helpers::fabric::{ServiceFactory, Context};
use exonum::messages::RawTransaction;
use exonum::storage::Snapshot;
use exonum::crypto::Hash;
use exonum::encoding;
use exonum::api::Api;
use iron::Handler;
use router::Router;

use timestamping::schema::TimestampingSchema;
use timestamping::transactions::TimestampingTransactions;
use timestamping::api::TimestampingApi;

pub const SERVICE_ID: u16 = 42;

pub struct TimestampingService;

/// `TimestampingService` instance which provides Api to make and validate timestamps of data
impl TimestampingService {
    pub fn default() -> TimestampingService {
        TimestampingService
    }
}

/// `Service` implementation for `TimestampingService`
impl Service for TimestampingService {
    fn service_name(&self) -> &'static str {
        "timestamping"
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    // Implement a method to deserialize transactions coming to the node.
    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
        let tx = TimestampingTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    // Hashes for the service tables that will be included into the state hash.
    // To simplify things, we don't have [Merkelized tables][merkle] in the service storage
    fn state_hash(&self, snapshot: &Snapshot) -> Vec<Hash> {
        let schema = TimestampingSchema::new(snapshot);
        schema.state_hash()
    }

    // Create a REST `Handler` to process web requests to the node.
    fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();
        let api = TimestampingApi::new(ctx);
        api.wire(&mut router);
        Some(Box::new(router))
    }
}

#[derive(Debug)]
pub struct TimestampingServiceFactory;

impl ServiceFactory for TimestampingServiceFactory {
    fn make_service(&mut self, _: &Context) -> Box<Service> {
        Box::new(TimestampingService::default())
    }
}
