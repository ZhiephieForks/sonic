// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Sonic OSS License v1.0 (SOSSL v1.0)

use std::net::SocketAddr;
use std::path::PathBuf;

pub fn server_log_level() -> String {
    "error".to_string()
}

pub fn channel_inet() -> SocketAddr {
    "[::1]:1491".parse().unwrap()
}

pub fn channel_tcp_timeout() -> u64 {
    300
}

pub fn channel_search_query_limit_default() -> u16 {
    10
}

pub fn channel_search_query_limit_maximum() -> u16 {
    100
}

pub fn channel_search_query_alternates_try() -> usize {
    4
}

pub fn channel_search_suggest_limit_default() -> u16 {
    5
}

pub fn channel_search_suggest_limit_maximum() -> u16 {
    20
}

pub fn store_kv_path() -> PathBuf {
    PathBuf::from("./data/store/kv/")
}

pub fn store_kv_retain_word_objects() -> usize {
    1000
}

pub fn store_kv_pool_inactive_after() -> u64 {
    1800
}

pub fn store_kv_database_compress() -> bool {
    true
}

pub fn store_kv_database_parallelism() -> u16 {
    2
}

pub fn store_kv_database_max_files() -> u16 {
    100
}

pub fn store_kv_database_max_compactions() -> u16 {
    1
}

pub fn store_kv_database_max_flushes() -> u16 {
    1
}

pub fn store_fst_path() -> PathBuf {
    PathBuf::from("./data/store/fst/")
}

pub fn store_fst_pool_inactive_after() -> u64 {
    300
}

pub fn store_fst_graph_consolidate_after() -> u64 {
    180
}
