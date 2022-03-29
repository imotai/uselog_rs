//
//
// uselog_test.rs
// Copyright (C) 2022 rtstore.io Author imrtstore <rtstore_dev@outlook.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//


#[cfg(test)]
mod tests {
    use uselog_rs::uselog;
    uselog!(info, debug, warn);
    #[test]
    fn it_works() {
        info!("test info {}", "xxx");
        debug!("test debug {}", "xxx");
        warn!("test warn {}", "xxx");
    }
}
