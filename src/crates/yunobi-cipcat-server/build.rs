// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 YunobiAirdrop

//! Generates the gRPC service from `proto/yunobi-cipcat.proto` when the `grpc`
//! feature is on. protox compiles the proto in Rust, so nothing needs
//! installing.

fn main() {
    #[cfg(feature = "grpc")]
    {
        let proto = "proto/yunobi-cipcat.proto";
        println!("cargo:rerun-if-changed={proto}");
        let descriptors = protox::compile([proto], ["proto"]).expect("yunobi-cipcat.proto compiles");
        tonic_prost_build::configure()
            .compile_fds(descriptors)
            .expect("the gRPC service generates");
    }
}
