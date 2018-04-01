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


use exonum::blockchain::{ApiContext, Blockchain, Transaction};
use exonum::encoding::serialize::FromHex;
use exonum::node::{TransactionSend, ApiSender};
use exonum::crypto::{Hash, PublicKey, SecretKey, hash};
use exonum::api::{Api, ApiError};
use iron::prelude::*;
use iron::status::Status;
use iron::headers::ContentType;
use iron::modifiers::Header;
use router::Router;

use bodyparser;
use serde_json;
use base64::STANDARD;

use super::schema::{Timestamp, TimestampingSchema};
use super::transactions::Tx;

/// `RestApi` instance for `TimestampingService`
#[derive(Clone)]
pub struct TimestampingApi {
    channel: ApiSender,
    blockchain: Blockchain,
    service_keys: (PublicKey, SecretKey)
}


impl TimestampingApi {
    /// Constructs a `TimestampingApi` for the given `context`.
    pub fn new(context: &ApiContext) -> TimestampingApi {
        let channel = context.node_channel().clone();
        let blockchain = context.blockchain().clone();
        let service_keys = (*context.public_key(), context.secret_key().clone());

        TimestampingApi {
            channel,
            blockchain,
            service_keys
        }
    }
}

/// POST Request message structure in the case of `data_hash` is provided
#[derive(Serialize, Deserialize, Clone)]
pub struct TransactionRequestHash {
    pub data_hash: Hash
}

/// POST Request message structure in the case of `data` (base64 encoded) is provided
base64_serde_type!(Base64Standard, STANDARD);
#[derive(Serialize, Deserialize, Clone)]
pub struct TransactionRequestBase64 {
    #[serde(with = "Base64Standard")]
    pub data: Vec<u8>
}


/// Response message structure in the case of `data` (base64 encoded) is provided
#[derive(Serialize, Deserialize,Clone)]
pub struct TransactionResponse {
    pub tx_hash: Hash,
    pub data_hash: Hash
}

impl TimestampingApi {
    /// Provides timestamp for given `data_hash` or `404 Not Found` in the case of timestamp
    /// for data for data doesn't exist.
    ///
    /// # Example
    /// GET: /api/services/timestamping/v0/timestamp/b900b8e9bba54eae47f6de08e8ff024e841274927d202a45551e875ced0eeb01
    ///
    /// {
    ///   "data_hash": "b900b8e9bba54eae47f6de08e8ff024e841274927d202a45551e875ced0eeb02",
    ///   "signature": "3cfd3c72d37183afbdf12b10dc6d605592ea9d4594a03315a631cfbadfd8e0f7",
    ///   "timestamp": "1522587390"
    /// }
    ///
    fn get_timestamp(&self, req: &mut Request) -> IronResult<Response> {
        let path = req.url.path();
        let document_hash = path.last().unwrap();
        let hash = Hash::from_hex(document_hash).map_err(|e| {
            IronError::new(e, (
                Status::BadRequest,
                Header(ContentType::json()),
                "\"Invalid request param: `data_hash`\"",
            ))
        })?;

        let snapshot = self.blockchain.snapshot();
        let schema = TimestampingSchema::new(snapshot);

        if let Some(timestamp) = schema.timestamp(&hash) {
            self.ok_response(&serde_json::to_value(timestamp).unwrap())
        } else {
            self.not_found_response(&serde_json::to_value("Timestamp not found").unwrap())
        }
    }

    /// Provides all stored timestamps, debug only method
    fn get_timestamps(&self, _: &mut Request) -> IronResult<Response> {
        let snapshot = self.blockchain.snapshot();
        let schema = TimestampingSchema::new(snapshot);
        let idx = schema.timestamps();
        let timestamps: Vec<Timestamp> = idx.values().collect();

        self.ok_response(&serde_json::to_value(&timestamps).unwrap())
    }

    /// Creates timestamp for given `data_hash`
    ///
    /// # Example
    /// POST: /api/services/timestamping/v0/timestamp/hash
    /// Request Payload:
    /// {
    ///   "data_hash": "b900b8e9bba54eae47f6de08e8ff024e841274927d202a45551e875ced0eeb02"
    /// }
    ///
    /// Response:
    /// {
    ///   "data_hash": "b900b8e9bba54eae47f6de08e8ff024e841274927d202a45551e875ced0eeb02",
    ///   "tx_hash": "d597703ee22849854ea8e9b322054e21d2ff15e9a10195681833976d83842d67"
    /// }
    ///
    fn post_hash(&self, req: &mut Request) -> IronResult<Response>{
        let hash: Hash = match req.get::<bodyparser::Struct<TransactionRequestHash>>() {
            Ok(Some(request)) => request.data_hash,
            Ok(None) => Err(ApiError::BadRequest("Empty request body".into()))?,
            Err(e) => Err(ApiError::BadRequest(e.to_string()))?,
        };
        self.send_tx(&hash)
    }

    /// Creates timestamp for given `data` (base64 encoded)
    fn post_base64(&self, req: &mut Request) -> IronResult<Response>{
        let hash: Hash = match req.get::<bodyparser::Struct<TransactionRequestBase64>>() {
            Ok(Some(request)) => hash(request.data.as_slice()),
            Ok(None) => Err(ApiError::BadRequest("Empty request body".into()))?,
            Err(e) => Err(ApiError::BadRequest(e.to_string()))?,
        };
        self.send_tx(&hash)
    }

    /// Common `send transaction` implementation
    fn send_tx(&self, hash: &Hash) -> IronResult<Response> {
        let tx = Tx::new(
            &self.service_keys.0,
            hash,
            &self.service_keys.1
        );
        let transaction: Box<Transaction> = tx.into();
        let tx_hash = transaction.hash();
        let data_hash = hash.clone();

        self.channel.send(transaction).map_err(ApiError::from)?;

        let json = TransactionResponse { tx_hash, data_hash };
        self.ok_response(&serde_json::to_value(&json).unwrap())
    }
}

/// `Api` trait implementation.
///
/// `Api` facilitates conversion between transactions/read requests and REST
/// endpoints; for example, it parses `POST`ed JSON into the binary transaction
/// representation used in Exonum internally.
impl Api for TimestampingApi {
    fn wire(&self, router: &mut Router) {
        let self_ = self.clone();
        let post_hash = move |req: &mut Request| self_.post_hash(req);
        let self_ = self.clone();
        let post_base64 = move |req: &mut Request| self_.post_base64(req);
        let self_ = self.clone();
        let get_timestamp = move |req: &mut Request| self_.get_timestamp(req);
        //let self_ = self.clone();
        //let get_timestamps = move |req: &mut Request| self_.get_timestamps(req);

        // Bind handlers to specific routes.
        router.post("/v0/timestamp/hash", post_hash, "post_hash_rt`");
        router.post("/v0/timestamp/base64", post_base64, "post_base64_rt`");
        router.get("/v0/timestamp/:data_hash", get_timestamp, "get_timestamp_rt");
        // optional
        //router.get("/v0/timestamps", get_timestamps, "get_timestamps_rt");
    }
}
