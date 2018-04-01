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

#![allow(dead_code)]

extern crate exonum;
extern crate exonum_time;
extern crate exonum_configuration;
extern crate lib;

use exonum::helpers::fabric::NodeBuilder;
use exonum::blockchain::{GenesisConfig, ValidatorKeys};
use exonum::node::{Node, NodeApiConfig, NodeConfig};
use exonum::storage::MemoryDB;
use exonum_time::{TimeService, TimeServiceFactory};
use lib::timestamping::{TimestampingService, TimestampingServiceFactory};
use lib::blockchain::{BlockchainService, BlockchainServiceFactory};

/// implementation to run `TimestampingService` and `BlockchainService` explorer service for dev-testing
fn dev_main() {
    fn node_config() -> NodeConfig {
        let (consensus_public_key, consensus_secret_key) = exonum::crypto::gen_keypair();
        let (service_public_key, service_secret_key) = exonum::crypto::gen_keypair();

        let validator_keys = ValidatorKeys {
            consensus_key: consensus_public_key,
            service_key: service_public_key,
        };
        let genesis :GenesisConfig = GenesisConfig::new(vec![validator_keys].into_iter());

        let api_address = "0.0.0.0:8000".parse().unwrap();
        let api_cfg = NodeApiConfig {
            public_api_address: Some(api_address),
            ..Default::default()
        };

        let peer_address = "0.0.0.0:2000".parse().unwrap();

        NodeConfig {
            listen_address: peer_address,
            peers: vec![],
            service_public_key,
            service_secret_key,
            consensus_public_key,
            consensus_secret_key,
            genesis,
            external_address: None,
            network: Default::default(),
            whitelist: Default::default(),
            api: api_cfg,
            mempool: Default::default(),
            services_configs: Default::default(),
            database: Default::default()
        }
    }

    exonum::helpers::init_logger().unwrap();

    let node = Node::new(
        MemoryDB::new(),
        vec![
            Box::new(TimeService::default()),
            Box::new(TimestampingService::default()),
            Box::new(BlockchainService::default())
        ],
        node_config(),
    );
    println!("Blockchain is ready...");
    node.run().unwrap();
}

/// implementation to run `TimestampingService` and `BlockchainService` as service
fn release() {
    NodeBuilder::new()
//        .with_service(Box::new(exonum_configuration::ServiceFactory))
        .with_service(Box::new(TimeServiceFactory))
        .with_service(Box::new(BlockchainServiceFactory))
        .with_service(Box::new(TimestampingServiceFactory))
        .run();
}

fn main() {
    release();
}
