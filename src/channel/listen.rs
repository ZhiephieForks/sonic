// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Sonic OSS License v1.0 (SOSSL v1.0)

use std::net::TcpListener;
use std::process;
use std::thread;

use super::handle::ChannelHandle;
use crate::{APP_CONF, THREAD_NAME_CHANNEL_CLIENT};

pub struct ChannelListenBuilder;
pub struct ChannelListen;

impl ChannelListenBuilder {
    pub fn new() -> ChannelListen {
        ChannelListen {}
    }
}

impl ChannelListen {
    pub fn run(&self) {
        match TcpListener::bind(APP_CONF.channel.inet) {
            Ok(listener) => {
                info!("listening on tcp://{}", APP_CONF.channel.inet);

                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            thread::Builder::new()
                                .name(THREAD_NAME_CHANNEL_CLIENT.to_string())
                                .spawn(move || {
                                    if let Ok(peer_addr) = stream.peer_addr() {
                                        debug!("channel client connecting: {}", peer_addr);
                                    }

                                    // Create client
                                    ChannelHandle::client(stream);
                                })
                                .ok();
                        }
                        Err(err) => {
                            warn!("error handling stream: {}", err);
                        }
                    }
                }
            }
            Err(err) => {
                error!("error binding channel listener: {}", err);

                // Exit Sonic
                process::exit(1);
            }
        }
    }
}
