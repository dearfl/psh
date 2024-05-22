// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of PSH.
//
// PSH is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// PSH is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.

use once_cell::sync::Lazy;

use super::{InterruptDetails, IrqDetails};
use crate::interrupt::raw::{parse_interrupts, parse_irq};
use crate::utils::Handle;

static INFO_GLOBAL: Lazy<Handle<Vec<IrqDetails>>> =
    Lazy::new(|| Handle::new(|| parse_irq!().map_err(Into::into)));

pub fn info_handle() -> Handle<Vec<IrqDetails>> {
    INFO_GLOBAL.clone()
}

static STAT_GLOBAL: Lazy<Handle<Vec<InterruptDetails>>> =
    Lazy::new(|| Handle::new(|| parse_interrupts!().map_err(Into::into)));

pub fn stat_handle() -> Handle<Vec<InterruptDetails>> {
    STAT_GLOBAL.clone()
}