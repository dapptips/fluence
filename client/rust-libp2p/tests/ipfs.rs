/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::error::Error;
use std::time::{Duration, Instant};

use bencher::stats::Stats;
use futures::stream::FuturesUnordered;
use futures_util::StreamExt;
use itertools::{Either, Itertools};

/*
    Plan:
        - Upload file to a single node (FOILED, todo)
        - Download it from all nodes
        - Measure
*/

// List of addresses (ip:port) of IPFS nodes
const NODES: [&str; 0] = [];

const FILES: [&str; 5] = [
    "QmdytmR4wULMd3SLo6ePF4s3WcRHWcpnJZ7bHhoj3QB13v",
    "QmSFxnK675wQ9Kc1uqWKyJUaNxvSc2BP5DbXCD3x93oq61",
    "QmdQEnYhrhgFKPCq5eKc7xb1k7rKyb3fGMitUPKvFAscVK",
    "QmR56UJmAaZLXLdTT1ALrE9vVqV8soUEekm9BMd4FnuYqV",
    "QmZa1tLVa8iEoDWbvwxV7rCxGnVGUff2jhF7a2DyCk5SbY",
];

fn pp(f: f64) -> u128 {
    Duration::from_secs_f64(f).as_millis()
}

#[tokio::test]
#[ignore]
async fn request() -> Result<(), Box<dyn Error>> {
    for file in FILES.iter() {
        let received: Vec<Result<f64, _>> = NODES
            .iter()
            // .cartesian_product(FILES.iter())
            .map(|&addr| {
                async move {
                    let start = Instant::now();
                    let addr = format!("http://{}/api/v0/object/get?arg={}", addr, file);
                    let resp = reqwest::get(&addr).await;
                    let elapsed = start.elapsed().as_secs_f64();
                    match resp {
                        Ok(_) => {
                            // let resp = resp.text().await?;
                            // println!("resp {} elpsd {}", resp, elapsed);
                            return Ok(elapsed);
                        }
                        Err(e) => return Err(e),
                    }
                }
            })
            .collect::<FuturesUnordered<_>>()
            .collect::<Vec<_>>()
            .await;

        let (received, err): (Vec<f64>, Vec<_>) = received.into_iter().partition_map(|r| match r {
            Ok(v) => Either::Left(v),
            Err(v) => Either::Right(v),
        });

        if received.is_empty() {
            println!("everything failed.");
            for err in err.iter().map(|e| e.to_string()).dedup() {
                println!("err: {}", err);
            }
        } else {
            println!(
                "count\tfailed\tmean\tmedian\tvar\t.75\t.95\t.99\tmax\tmin\n{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                received.len(),
                err.len(),
                pp(received.mean()),
                pp(received.median()),
                pp(received.var()),
                pp(received.percentile(75.0)),
                pp(received.percentile(95.0)),
                pp(received.percentile(99.0)),
                pp(received.max()),
                pp(received.min())
            );
        }
    }
    Ok(())
}
