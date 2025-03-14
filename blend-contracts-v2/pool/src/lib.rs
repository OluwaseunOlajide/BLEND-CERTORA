#![no_std]

#[cfg(any(test, feature = "testutils"))]
extern crate std;

#[cfg(any(test, feature = "testutils"))]
pub use pool::{Pool as PoolState, PositionData, Reserve};

mod auctions;
mod constants;
mod contract;
mod dependencies;
mod emissions;
mod errors;
mod events;
mod pool;
mod storage;
mod testutils;
mod validator;

pub use auctions::{AuctionData, AuctionType};
pub use contract::*;
pub use emissions::ReserveEmissionMetadata;
pub use errors::PoolError;
pub use pool::{FlashLoan, Positions, Request, RequestType};
pub use storage::{
    AuctionKey, PoolConfig, PoolDataKey, PoolEmissionConfig, ReserveConfig, ReserveData,
    ReserveEmissionData, UserEmissionData, UserReserveKey,
};
