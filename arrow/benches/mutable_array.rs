// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate arrow;

use arrow::{array::*, util::bench_util::create_string_array};

fn bench_multiple<T: Array>(arrays: &Vec<T>) {
    let array_data = arrays
        .iter()
        .map(|a| a.data_ref())
        .collect::<Vec<&ArrayData>>();
    let capacity = array_data.iter().map(|data| data.len()).sum();
    let mut mutable = MutableArrayData::new(array_data.clone(), false, capacity);
    array_data
        .iter()
        .enumerate()
        .for_each(|(i, data)| mutable.extend(i, 0, data.len()));
    mutable.freeze();
}

fn add_benchmark(c: &mut Criterion) {
    let arrays = vec![
        create_string_array::<i32>(1024, 0.5),
        create_string_array::<i32>(1024, 0.5),
        create_string_array::<i32>(1024, 0.5),
        create_string_array::<i32>(1024, 0.5),
        create_string_array::<i32>(1024, 0.5),
        create_string_array::<i32>(1024, 0.5),
        create_string_array::<i32>(1024, 0.5),
        create_string_array::<i32>(1024, 0.5),
        create_string_array::<i32>(1024, 0.5),
    ];
    c.bench_function("mutable str multiple 1024", |b| {
        b.iter(|| bench_multiple(&arrays))
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
