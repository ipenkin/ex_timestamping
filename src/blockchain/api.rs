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

use exonum::api::Api;
use exonum::blockchain::Blockchain;
use exonum::crypto::Hash;
use exonum::encoding::serialize::FromHex;
use exonum::node::ApiSender;

use iron::prelude::*;
use iron::status::Status;
use iron::headers::ContentType;
use iron::modifiers::Header;
use router::Router;
use exonum::explorer::BlockchainExplorer;
use serde_json;
use exonum::helpers::Height;

/// `RestApi` instance for `BlockchainService`
#[derive(Clone)]
pub struct BlockchainApi {
    channel: ApiSender,
    blockchain: Blockchain,
}

impl BlockchainApi {
    /// Constructs a `BlockchainApi` for the given `channel` and `blockchain`.
    pub fn new(channel: ApiSender, blockchain: Blockchain) -> BlockchainApi {
        BlockchainApi {
            channel,
            blockchain,
        }
    }
}

impl BlockchainApi {
    /// Provides actual blockchain height
    fn get_blockchain_height(&self, _: &mut Request) -> IronResult<Response> {
        let explorer = BlockchainExplorer::new(&self.blockchain);
        self.ok_response(&serde_json::to_value(explorer.height()).unwrap())
    }

    /// Provides block content and transactions list by block number
    ///
    /// # Examples
    ///    GET: /api/services/blockchain/v0/block/104
    ///
    ///    {
    ///        "block": {
    ///            "height": "104",
    ///            "prev_hash": "270cac720fa15e3e4f419c5ee6548a827ab63a5f1435cb82e6aef368dd66aac1",
    ///            "proposer_id": 0,
    ///            "schema_version": 0,
    ///            "state_hash": "494aad039ba28340c57af63688d477ac5ce8bf8fccb9549c42ead3974c9b3ef9",
    ///            "tx_count": 2,
    ///            "tx_hash": "6f4ad4d2291d3fc41d77840a153f808235a4b67b0dd6245e145c21ed4029371f"
    ///        },
    ///        "precommits": [
    ///            {
    ///                "body": {
    ///                    "block_hash": "691625abe58bcf7b824d53f212761accb481140b4159b210d85839db4e9fbea4",
    ///                    "height": "104",
    ///                    "propose_hash": "2c57d47d2a4fcc02904b9f2819d740fc1fbb426ef4902e7fcbe9b927b6be1f33",
    ///                    "round": 1,
    ///                    "time": {
    ///                        "nanos": 97101219,
    ///                        "secs": "1522530663"
    ///                    },
    ///                    "validator": 0
    ///                },
    ///                "message_id": 4,
    ///                "protocol_version": 0,
    ///                "service_id": 0,
    ///                "signature": "2f23e7a4876fa9a891ba25e57d07ea511e920286951d28e82ce008b46d42ffc466c469ad0d8d82c8ec43200e10c87c14409174b4fbce61b6ad99af8d02c8fc0e"
    ///            }
    ///        ],
    ///        "txs": [
    ///            "a763d3adba6becdf276757f36542ca1af3dbb3fd9f3f6776015372362faf9f4c",
    ///            "b79a70e327de70df58e563622f81b70bf8dcb384e51b9f03d8cbd3b541b74db9"
    ///        ]
    ///    }
    fn get_block(&self, req: &mut Request) -> IronResult<Response> {
        let path = req.url.path();
        let block_num = path.last().unwrap();
        let num = u64::from_str_radix(block_num, 10u32).map_err(|e| {
            IronError::new(e, (
                Status::BadRequest,
                Header(ContentType::json()),
                "\"Invalid request param: `num`\"",
            ))
        })?;;

        let explorer = BlockchainExplorer::new(&self.blockchain);
        if let Some(block) = explorer.block(Height(num)) {
            self.ok_response(&serde_json::to_value(block).unwrap())
        } else {
            self.not_found_response(&serde_json::to_value("Block is not found").unwrap())
        }
    }

    /// Provides transaction content by `tx_hash` number
    ///
    /// # Examples
    ///   GET: /api/services/blockchain/v0/transaction/a763d3adba6becdf276757f36542ca1af3dbb3fd9f3f6776015372362faf9f4c
    ///
    ///    {
    ///        "content": {
    ///            "body": {
    ///                "doc_hash": "6ce29b2d3ecadc434107ce52c287001c968a1b6eca3e5a1eb62a2419e2924b45",
    ///                "doc_metainfo": "Document v_1",
    ///                "pub_key": "6ce29b2d3ecadc434107ce52c287001c968a1b6eca3e5a1eb62a2419e2924235"
    ///            },
    ///            "message_id": 0,
    ///            "protocol_version": 0,
    ///            "service_id": 42,
    ///            "signature": "9f684227f1de663775848b3db656bca685e085391e2b00b0e115679fd45443ef58a5abeb555ab3d5f7a3cd27955a2079e5fd486743f36515c8e5bea07992100b"
    ///        },
    ///        "location": {
    ///            "block_height": "104",
    ///            "position_in_block": "0"
    ///        },
    ///        "location_proof": {
    ///            "left": {
    ///                "val": "a763d3adba6becdf276757f36542ca1af3dbb3fd9f3f6776015372362faf9f4c"
    ///            },
    ///            "right": "b79a70e327de70df58e563622f81b70bf8dcb384e51b9f03d8cbd3b541b74db9"
    ///        },
    ///        "status": {
    ///            "type": "success"
    ///        },
    ///        "type": "committed"
    ///    }
    fn get_transaction(&self, req: &mut Request) -> IronResult<Response> {
        let path = req.url.path();
        let tx_hash = path.last().unwrap();
        let hash = Hash::from_hex(tx_hash).map_err(|e| {
            IronError::new(e, (
                Status::BadRequest,
                Header(ContentType::json()),
                "\"Invalid request param: `hash`\"",
            ))
        })?;
        let explorer = BlockchainExplorer::new(&self.blockchain);
        if let Some(transaction) = explorer.transaction(&hash) {
            self.ok_response(&serde_json::to_value(transaction).unwrap())
        } else {
            self.not_found_response(&serde_json::to_value("Transaction is not found").unwrap())
        }
    }

}

/// `Api` trait implementation.
///
/// `Api` facilitates conversion between read requests and REST endpoints;
/// representation used in Exonum internally.
impl Api for BlockchainApi {
    fn wire(&self, router: &mut Router) {
        let self_ = self.clone();
        let get_blockchain_height = move |req: &mut Request| self_.get_blockchain_height(req);
        let self_ = self.clone();
        let get_block = move |req: &mut Request| self_.get_block(req);
        let self_ = self.clone();
        let get_tx = move |req: &mut Request| self_.get_transaction(req);

        // Bind handlers to specific routes.
        router.get("/v0/height", get_blockchain_height, "get_blockchain_height");
        router.get("/v0/block/:num", get_block, "get_block");
        router.get("/v0/transaction/:hash", get_tx, "get_tx");
    }
}
