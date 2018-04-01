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


use exonum::storage::{Fork, ProofMapIndex, Snapshot, StorageValue};
use chrono::{DateTime, Utc};
use exonum::crypto::{Hash, hash};


/// Base `TimestampingService` storage structure
encoding_struct! {
    struct Timestamp {
        /// (data_hash + timestamp) hashed
        signature: &Hash,
        /// timestamp
        timestamp: i64,
        /// data hash
        data_hash: &Hash,
    }
}


/// imlementation for `Timestamp` creation
impl Timestamp {
    pub fn from_parts(datetime: &DateTime<Utc>, data_hash: &Hash) -> Timestamp {
        Timestamp::new(
            &Timestamp::sign(data_hash, datetime),
            datetime.timestamp(),
            data_hash
        )
    }

    fn sign(data_hash: &Hash, datetime: &DateTime<Utc>) -> Hash {
        let mut bytes: Vec<u8> = data_hash.into_bytes();
        bytes.append(& mut datetime.timestamp().into_bytes());
        hash(bytes.as_slice())
    }
}

pub struct TimestampingSchema<T> {
    view: T,
}

impl<T: AsRef<Snapshot>> TimestampingSchema<T> {
    pub fn new(view: T) -> Self {
        TimestampingSchema { view }
    }

    pub fn timestamps(&self) -> ProofMapIndex<&Snapshot, Hash, Timestamp> {
        ProofMapIndex::new("timestamping_service.timestamps", self.view.as_ref())
    }

    pub fn timestamp(&self, stamp: &Hash) -> Option<Timestamp> {
        self.timestamps().get(stamp)
    }

    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.timestamps().merkle_root()]
    }
}

impl<'a> TimestampingSchema<&'a mut Fork> {
    /// Returns a mutable version of the wallets table.
    pub fn timestamps_mut(&mut self) -> ProofMapIndex<&mut Fork, Hash, Timestamp> {
        ProofMapIndex::new("timestamping_service.timestamps", &mut self.view)
    }
}

