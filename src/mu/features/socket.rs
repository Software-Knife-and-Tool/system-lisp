//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(clippy::useless_conversion, unused_imports)]
//! socket feature
#[rustfmt::skip]
use {
    crate::{
        core::{
            apply::Apply as _,
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            tag::Tag,
            type_::Type,
        },
        features::feature::Feature,
        types::{
            cons::Cons,
            fixnum::Fixnum,
            float::Float,
            struct_::Struct,
            symbol::Symbol,
            vector::{Vector, VectorType},
        },
    },
    socket,
};

pub trait Socket {
    fn feature() -> Feature;
}

impl Socket for Feature {
    fn feature() -> Feature {
        Feature {
            functions: Some(vec![
                ("accept", 2, Feature::socket_accept),
                ("bind", 9, Feature::socket_bind),
                ("close", 9, Feature::socket_close),
                ("connect", 1, Feature::socket_connect),
                ("fcntl", 9, Feature::socket_fcntl),
                ("getpeername", 9, Feature::socket_getpeername),
                ("getsockname", 9, Feature::socket_getsockname),
                ("getsockopt", 9, Feature::socket_getsockopt),
                ("ioctl", 9, Feature::socket_ioctl),
                ("listen", 0, Feature::socket_listen),
                ("read", 9, Feature::socket_read),
                ("recv", 9, Feature::socket_recv),
                ("select", 9, Feature::socket_select),
                ("send", 9, Feature::socket_send),
                ("shutdown", 9, Feature::socket_shutdown),
                ("socket", 3, Feature::socket_socket),
                ("socketpair", 9, Feature::socket_socketpair),
                ("write", 9, Feature::socket_write),
            ]),
            symbols: Some(vec![
                ("AF_UNIX", socket::AF_UNIX.into()),
                ("AF_INET", socket::AF_INET.into()),
                ("AF_INET6", socket::AF_INET6.into()),
                ("IPPROTO_IP", socket::IPPROTO_IP.into()),
                ("IPPROTO_IPV6", socket::IPPROTO_IPV6.into()),
                ("IPPROTO_RAW", socket::IPPROTO_RAW.into()),
                ("IPPROTO_TCP", socket::IPPROTO_TCP.into()),
                ("IPV6_ADD_MEMBERSHIP", socket::IPV6_ADD_MEMBERSHIP.into()),
                ("IPV6_DROP_MEMBERSHIP", socket::IPV6_DROP_MEMBERSHIP.into()),
                ("IP_ADD_MEMBERSHIP", socket::IP_ADD_MEMBERSHIP.into()),
                ("IP_DROP_MEMBERSHIP", socket::IP_DROP_MEMBERSHIP.into()),
                ("IP_HDRINCL", socket::IP_HDRINCL.into()),
                ("IP_MULTICAST_LOOP", socket::IP_MULTICAST_LOOP.into()),
                ("IP_MULTICAST_TTL", socket::IP_MULTICAST_TTL.into()),
                ("IP_TTL", socket::IP_TTL.into()),
                ("SHUT_RD", socket::SHUT_RD.into()),
                ("SHUT_WR", socket::SHUT_WR.into()),
                ("SOCK_DGRAM", socket::SOCK_DGRAM.into()),
                ("SOCK_RAW", socket::SOCK_RAW.into()),
                ("SOCK_STREAM", socket::SOCK_STREAM.into()),
                ("SOL_SOCKET", socket::SOL_SOCKET.into()),
                ("SO_BROADCAST", socket::SO_BROADCAST.into()),
                ("SO_ERROR", socket::SO_ERROR.into()),
                ("SO_KEEPALIVE", socket::SO_KEEPALIVE.into()),
                ("SO_REUSEADDR", socket::SO_REUSEADDR.into()),
                ("TCP_NODELAY", socket::TCP_NODELAY.into()),
            ]),
            namespace: "feature/socket".into(),
        }
    }
}

pub trait CoreFn {
    fn socket_accept(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_bind(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_close(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_connect(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_fcntl(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_getpeername(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_getsockname(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_getsockopt(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_ioctl(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_listen(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_read(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_recv(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_select(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_send(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_shutdown(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_socket(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_socketpair(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn socket_write(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Feature {
    fn socket_accept(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_bind(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_close(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_connect(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_fcntl(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_getpeername(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_getsockname(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_getsockopt(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_ioctl(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_listen(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_read(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_recv(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_select(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_send(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_shutdown(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_socket(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check(
            "feature/socket:socket",
            &[Type::Fixnum, Type::Fixnum, Type::Fixnum],
            fp,
        )?;

        Ok(())
    }
    fn socket_socketpair(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
    fn socket_write(_env: &Env, _fp: &mut Frame) -> exception::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
