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


use tempdir::TempDir;
use futures::sync::mpsc;
use test::Bencher;
use exonum::storage::{Database, Patch, RocksDB, DbOptions};
use exonum::blockchain::{Blockchain, Transaction, Schema, Service};
use exonum::crypto::{gen_keypair, CryptoHash, Hash, PublicKey, SecretKey};
use exonum::messages::Message;
use exonum::helpers::{Height, ValidatorId};
use exonum::node::ApiSender;
use exonum_time::TimeService;

use timestamping::transactions::Tx;
use timestamping::TimestampingService;

#[bench]
fn bench_execute_block_timestamping_tx_1_h_100(b: &mut Bencher) {
    bench_execute_block_timestamping_rocksdb(b, 1, 100)
}

#[bench]
fn bench_execute_block_timestamping_tx_2_h_100(b: &mut Bencher) {
    bench_execute_block_timestamping_rocksdb(b, 2, 100)
}

#[bench]
fn bench_execute_block_timestamping_tx_200_h_100(b: &mut Bencher) {
    bench_execute_block_timestamping_rocksdb(b, 200, 100)
}

#[bench]
fn bench_execute_block_timestamping_tx_400_h_100(b: &mut Bencher) {
    bench_execute_block_timestamping_rocksdb(b, 400, 100)
}

#[bench]
fn bench_execute_block_timestamping_tx_600_h_100(b: &mut Bencher) {
    bench_execute_block_timestamping_rocksdb(b, 600, 100)
}

#[bench]
fn bench_execute_block_timestamping_tx_800_h_100(b: &mut Bencher) {
    bench_execute_block_timestamping_rocksdb(b, 800, 100)
}

#[bench]
fn bench_execute_block_timestamping_tx_1000_h_100(b: &mut Bencher) {
    bench_execute_block_timestamping_rocksdb(b, 1000, 100)
}

#[bench]
fn bench_execute_block_timestamping_tx_1000_h_1(b: &mut Bencher) {
    bench_execute_block_timestamping_rocksdb(b, 1000, 1)
}

#[bench]
fn bench_execute_block_timestamping_tx_1000_h_20(b: &mut Bencher) {
    bench_execute_block_timestamping_rocksdb(b, 1000, 20)
}

#[bench]
fn bench_execute_block_timestamping_tx_1000_h_40(b: &mut Bencher) {
    bench_execute_block_timestamping_rocksdb(b, 1000, 40)
}

#[bench]
fn bench_execute_block_timestamping_tx_1000_h_60(b: &mut Bencher) {
    bench_execute_block_timestamping_rocksdb(b, 1000, 60)
}

#[bench]
fn bench_execute_block_timestamping_tx_1000_h_80(b: &mut Bencher) {
    bench_execute_block_timestamping_rocksdb(b, 1000, 80)
}

fn bench_execute_block_timestamping_rocksdb(b: &mut Bencher, tx_count: u64, height: u64) {
    let tempdir = TempDir::new("exonum").unwrap();
    let db = create_rocksdb(&tempdir);
    execute_timestamping(db, b, tx_count, height)
}

fn execute_timestamping(db: Box<Database>, b: &mut Bencher, tx_count: u64, height: u64) {

    fn create_blockchain(db: Box<Database>, services: Vec<Box<Service>>) -> Blockchain {
        let dummy_channel = mpsc::channel(1);
        let dummy_keypair = (PublicKey::zero(), SecretKey::zero());
        Blockchain::new(
            db,
            services,
            dummy_keypair.0,
            dummy_keypair.1,
            ApiSender::new(dummy_channel.0),
        )
    }

    fn execute_block(blockchain: &Blockchain, height: u64, txs: &[Hash]) -> Patch {
        blockchain
            .create_patch(ValidatorId::zero(), Height(height), txs)
            .1
    }

    fn prepare_txs(blockchain: &mut Blockchain, height: u64, count: u64) -> Vec<Hash> {
        let mut fork = blockchain.fork();
        let mut txs = Vec::new();
        {
            let mut schema = Schema::new(&mut fork);
            let (pub_key, sec_key) = gen_keypair();
            for i in (height * count)..((height + 1) * count) {
                let tx = Tx::new(&pub_key, &i.hash(), &sec_key);
                let tx_hash = Transaction::hash(&tx);
                txs.push(tx_hash);
                schema.add_transaction_into_pool(tx.raw().clone());
            }
        }
        blockchain.merge(fork.into_patch()).unwrap();
        txs
    }
    let mut blockchain = create_blockchain(
        db,
        vec![Box::new(TimestampingService::default()), Box::new(TimeService::default())]
    );
    for i in 0..height {
        let txs = prepare_txs(&mut blockchain, i, tx_count);
        let patch = execute_block(&blockchain, i, &txs);
        blockchain.merge(patch).unwrap();
    }

    let txs = prepare_txs(&mut blockchain, height, tx_count);

    b.iter(|| execute_block(&blockchain, height, &txs));
}

fn create_rocksdb(tempdir: &TempDir) -> Box<Database> {
    let options = DbOptions::default();
    let db = Box::new(RocksDB::open(tempdir.path(), &options).unwrap());
    db as Box<Database>
}