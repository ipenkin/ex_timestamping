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

use exonum::blockchain::Transaction;
use exonum::storage::Fork;
use exonum::blockchain::ExecutionResult;


// stub
transactions! {
    pub BlockchainTransactions {
        const SERVICE_ID = super::SERVICE_ID;

        struct StubTx {
            stub: &str
        }
    }
}

impl Transaction for StubTx {
    fn verify(&self) -> bool {
        true
    }

    fn execute(&self, _: &mut Fork) -> ExecutionResult {
        Ok(())
    }
}