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

use clap::Parser;

#[derive(Parser, Debug)]
#[non_exhaustive]
pub struct Args {
    // if psh runs in netdata_plugin mode?
    #[clap(skip)]
    pub netdata_plugin: Option<bool>,

    /// frequency that's passed by netdata if run as netdata plugin.
    pub netdata_freq: u64,

    #[arg(verbatim_doc_comment)]
    /// Path to the install script
    #[arg(long)]
    #[arg(value_name = "PATH")]
    pub install: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Path to the get_sysinfo script
    #[arg(long)]
    #[arg(value_name = "PATH")]
    pub get_sysinfo: Option<String>,
}