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


use exonum::blockchain::{Transaction, ExecutionResult};
use exonum::messages::Message;
use exonum::storage::Fork;
use exonum_time::TimeSchema;


use super::schema::{Timestamp, TimestampingSchema};
use super::transactions::Tx;
use super::errors::Error;

/// Implementation of `Transaction` trait for `Tx`
impl Transaction for Tx {
    /// Verifies the internal consistency of the transaction.
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    /// Receives a fork of the current blockchain state and put new `Timestamp`
    /// by given `data_hash` as id.
    ///
    /// Timestamp {
    ///      signature: hash(data_hash + timestamp)
    ///      timestamp: exonum_time::TimeService.consolidated_time()
    ///      data_hash: data_hash
    /// }
    ///
    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let time = TimeSchema::new(&view).time().get();
        let mut schema = TimestampingSchema::new(view);

        match time {
            Some(current_time) => {
                if schema.timestamp(self.doc_hash()).is_none() {
                    let timestamp = Timestamp::from_parts(&current_time, self.doc_hash());
                    schema.timestamps_mut().put(self.doc_hash(), timestamp);
                    Ok(())
                } else {
                    Err(Error::DocumentAlreadyExists)?
                }
            }
            _ => {
                Err(Error::TimeServiceError)?
            }
        }
    }
}







