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


pub mod transactions;
mod api;


use exonum::api::Api;
use exonum::blockchain::{Service, ApiContext, Transaction, TransactionSet};
use exonum::crypto::Hash;
use exonum::encoding;
use exonum::helpers::fabric::{ServiceFactory, Context};
use exonum::messages::RawTransaction;
use exonum::storage::Snapshot;


use iron::Handler;
use router::Router;

use blockchain::api::BlockchainApi;
use blockchain::transactions::BlockchainTransactions;

pub const SERVICE_ID: u16 = 2;

pub struct BlockchainService;


/// `BlockchainService` instance which provides Api to explore blockchain state
impl BlockchainService {
    pub fn default() -> BlockchainService {
        BlockchainService
    }
}

/// `Service` implementation for `BlockchainService`
impl Service for BlockchainService {
    fn service_name(&self) -> &'static str {
        "blockchain"
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    /// Implement a method to deserialize transactions coming to the node.
    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
        let tx = BlockchainTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    /// Hashes for the service tables that will be included into the state hash.
    /// To simplify things, we don't have [Merkelized tables][merkle] in the service storage
    fn state_hash(&self, _: &Snapshot) -> Vec<Hash> {
        vec![]
    }

    /// Create a REST `Handler` to process web requests to the node.
    fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();
        let api = BlockchainApi::new(ctx.node_channel().clone(), ctx.blockchain().clone());
        api.wire(&mut router);
        Some(Box::new(router))
    }
}

/// Factory instance for `BlockchainService` creation.
#[derive(Debug)]
pub struct BlockchainServiceFactory;


/// Factory implementation
impl ServiceFactory for BlockchainServiceFactory {
    fn make_service(&mut self, _: &Context) -> Box<Service> {
        Box::new(BlockchainService::default())
    }
}
