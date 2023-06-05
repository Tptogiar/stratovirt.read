// Copyright (c) 2022 Huawei Technologies Co.,Ltd. All rights reserved.
//
// StratoVirt is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//         http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MachineError {
    #[error("AddressSpace")]
    AddressSpace {
        #[from]
        source: address_space::error::AddressSpaceError,
    },
    #[error("IntCtrl")]
    #[cfg(target_arch = "aarch64")]
    IntCtrl {
        #[from]
        source: devices::IntCtrlErrs,
    },
    #[error("Legacy")]
    Legacy {
        #[from]
        source: devices::legacy::error::LegacyError,
    },
    #[error("MicroVm")]
    MicroVm {
        #[from]
        source: super::micro_vm::error::MicroVmError,
    },
    #[error("StdVm")]
    StdVm {
        #[from]
        source: super::standard_vm::error::StandardVmError,
    },
    #[error("Util")]
    Util {
        #[from]
        source: util::error::UtilError,
    },
    #[error("Virtio")]
    Virtio {
        #[from]
        source: virtio::error::VirtioError,
    },
    #[error("MachineManager")]
    MachineManager {
        #[from]
        source: machine_manager::config::error::ConfigError,
    },
    #[error("Hypervisor")]
    Hypervisor {
        #[from]
        source: hypervisor::error::HypervisorError,
    },
    #[error("Io")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("Failed to add {0} device.")]
    AddDevErr(String),
    #[error("Failed to load kernel.")]
    LoadKernErr,
    #[error("Failed to create memory address space")]
    CrtMemSpaceErr,
    #[error("Failed to create I/O address space")]
    CrtIoSpaceErr,
    #[error("Failed to register region in memory space: base={0},size={1}")]
    RegMemRegionErr(u64, u64),
    #[error("Failed to init eventfd {0}.")]
    InitEventFdErr(String),
    #[error("Failed to realize virtio mmio.")]
    RlzVirtioMmioErr,
    #[error("Failed to create irq chip.")]
    #[cfg(target_arch = "x86_64")]
    CrtIrqchipErr,
    #[error("Failed to set identity map address.")]
    #[cfg(target_arch = "x86_64")]
    SetIdentityMapAddr,
    #[error("Failed to set tss address.")]
    #[cfg(target_arch = "x86_64")]
    SetTssErr,
    #[error("Failed to create PIT.")]
    #[cfg(target_arch = "x86_64")]
    CrtPitErr,
    #[error("Failed to generate FDT.")]
    #[cfg(target_arch = "aarch64")]
    GenFdtErr,
    #[error("Failed to write FDT: addr={0}, size={1}")]
    #[cfg(target_arch = "aarch64")]
    WrtFdtErr(u64, usize),
    #[error("Failed to register event notifier.")]
    RegNotifierErr,
    #[error("Failed to run vcpu{0}.")]
    StartVcpuErr(u8),
    #[error("Failed to pause vcpu{0}.")]
    PauseVcpuErr(u8),
    #[error("Failed to resume vcpu{0}")]
    ResumeVcpuErr(u8),
    #[error("Failed to destroy vcpu{0}.")]
    DestroyVcpuErr(u8),
}
