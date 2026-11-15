// SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Transmissions outstanding, reported in the order they were handed over
//! whatever order they finish in, as can-hal's `ConfirmedTransmit` requires:
//! can-common's ordering, with can-hal's ids and outcomes.

use can_common::ordering::Done;
use can_hal::{Abort, Failure, Outcome, TransmitId};

/// bxCAN's transmit mailboxes, and so the transmissions outstanding at once.
pub(crate) const MAILBOXES: usize = 3;

/// The transmissions outstanding.
pub(crate) type Outstanding<T> = can_common::ordering::Outstanding<TransmitId, T, MAILBOXES>;

/// How a transmission ended, as can-hal reports it.
pub(crate) fn outcome<T>(done: Done<T>) -> Outcome<T> {
    match done {
        Done::Sent(time) => Outcome::Sent(time),
        Done::Aborted => Outcome::Aborted(Abort::Confirmed),
        Done::GaveUp => Outcome::Failed(Failure::GaveUp),
    }
}
