#![no_std]

mod impl_items_view_;
mod items_view_;
mod array_;

#[cfg(test)]
extern crate std;

pub use array_::{TrArray, TrAsSlice, TrAsSliceMut};
pub use items_view_::{TrItemsRefView, TrItemsMutView};
