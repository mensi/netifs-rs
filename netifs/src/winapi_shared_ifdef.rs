// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

// Unfortunately it looks like winapi upstream is barely maintained, so here is a
// copy of winapi::shared::ifdef from git until it gets released to crates.io.

use winapi::{STRUCT, ENUM};
use core;

macro_rules! UNION {
    ($(#[$attrs:meta])* union $name:ident {
        [$stype:ty; $ssize:expr],
        $($variant:ident $variant_mut:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])*
        pub struct $name([$stype; $ssize]);
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
        impl $name {$(
            #[inline]
            pub unsafe fn $variant(&self) -> &$ftype {
                &*(self as *const _ as *const $ftype)
            }
            #[inline]
            pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
                &mut *(self as *mut _ as *mut $ftype)
            }
        )+}
    );
    ($(#[$attrs:meta])* union $name:ident {
        [$stype32:ty; $ssize32:expr] [$stype64:ty; $ssize64:expr],
        $($variant:ident $variant_mut:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])* #[cfg(target_pointer_width = "32")]
        pub struct $name([$stype32; $ssize32]);
        #[repr(C)] $(#[$attrs])* #[cfg(target_pointer_width = "64")]
        pub struct $name([$stype64; $ssize64]);
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
        impl $name {$(
            #[inline]
            pub unsafe fn $variant(&self) -> &$ftype {
                &*(self as *const _ as *const $ftype)
            }
            #[inline]
            pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
                &mut *(self as *mut _ as *mut $ftype)
            }
        )+}
    );
}

macro_rules! BITFIELD {
    ($base:ident $field:ident: $fieldtype:ty [
        $($thing:ident $set_thing:ident[$r:expr],)+
    ]) => {
        impl $base {$(
            #[inline]
            pub fn $thing(&self) -> $fieldtype {
                let size = core::mem::size_of::<$fieldtype>() * 8;
                self.$field << (size - $r.end) >> (size - $r.end + $r.start)
            }
            #[inline]
            pub fn $set_thing(&mut self, val: $fieldtype) {
                let mask = ((1 << ($r.end - $r.start)) - 1) << $r.start;
                self.$field &= !mask;
                self.$field |= (val << $r.start) & mask;
            }
        )+}
    }
}

use winapi::shared::basetsd::{UINT16, UINT32, ULONG32, ULONG64};
use winapi::shared::guiddef::GUID;
use winapi::shared::ntdef::{BOOLEAN, UCHAR, ULONG, USHORT, WCHAR};

pub type NET_IF_COMPARTMENT_ID = UINT32;
pub type PNET_IF_COMPARTMENT_ID = *mut NET_IF_COMPARTMENT_ID;
pub const NET_IF_COMPARTMENT_ID_UNSPECIFIED: NET_IF_COMPARTMENT_ID = 0;
pub const NET_IF_COMPARTMENT_ID_PRIMARY: NET_IF_COMPARTMENT_ID = 1;
pub type NET_IF_NETWORK_GUID = GUID;
pub type PNET_IF_NETWORK_GUID = *mut NET_IF_NETWORK_GUID;
ENUM!{enum NET_IF_OPER_STATUS {
    NET_IF_OPER_STATUS_UP = 1,
    NET_IF_OPER_STATUS_DOWN = 2,
    NET_IF_OPER_STATUS_TESTING = 3,
    NET_IF_OPER_STATUS_UNKNOWN = 4,
    NET_IF_OPER_STATUS_DORMANT = 5,
    NET_IF_OPER_STATUS_NOT_PRESENT = 6,
    NET_IF_OPER_STATUS_LOWER_LAYER_DOWN = 7,
}}
pub type PNET_IF_OPER_STATUS = *mut NET_IF_OPER_STATUS;
pub type NET_IF_OBJECT_ID = ULONG32;
pub type PNET_IF_OBJECT_ID = *mut NET_IF_OBJECT_ID;
ENUM!{enum NET_IF_ADMIN_STATUS {
    NET_IF_ADMIN_STATUS_UP = 1,
    NET_IF_ADMIN_STATUS_DOWN = 2,
    NET_IF_ADMIN_STATUS_TESTING = 3,
}}
pub type PNET_IF_ADMIN_STATUS = *mut NET_IF_ADMIN_STATUS;
pub type NET_IF_COMPARTMENT_SCOPE = UINT32;
pub type PNET_IF_COMPARTMENT_SCOPE = *mut NET_IF_COMPARTMENT_SCOPE;
pub const NET_IF_COMPARTMENT_SCOPE_UNSPECIFIED: NET_IF_COMPARTMENT_SCOPE = 0;
pub const NET_IF_COMPARTMENT_SCOPE_ALL: NET_IF_COMPARTMENT_SCOPE = -1i32 as u32;
ENUM!{enum NET_IF_RCV_ADDRESS_TYPE {
    NET_IF_RCV_ADDRESS_TYPE_OTHER = 1,
    NET_IF_RCV_ADDRESS_TYPE_VOLATILE = 2,
    NET_IF_RCV_ADDRESS_TYPE_NON_VOLATILE = 3,
}}
pub type PNET_IF_RCV_ADDRESS_TYPE = *mut NET_IF_RCV_ADDRESS_TYPE;
STRUCT!{struct NET_IF_RCV_ADDRESS_LH {
    ifRcvAddressType: NET_IF_RCV_ADDRESS_TYPE,
    ifRcvAddressLength: USHORT,
    ifRcvAddressOffset: USHORT,
}}
pub type PNET_IF_RCV_ADDRESS_LH = *mut NET_IF_RCV_ADDRESS_LH;
STRUCT!{struct NET_IF_ALIAS_LH {
    ifAliasLength: USHORT,
    ifAliasOffset: USHORT,
}}
pub type PNET_IF_ALIAS_LH = *mut NET_IF_ALIAS_LH;
STRUCT!{struct NET_LUID_LH_Info {
    bitfield: ULONG64,
}}
BITFIELD!{NET_LUID_LH_Info bitfield: ULONG64 [
    Reserved set_Reserved[0..24],
    NetLuidIndex set_NetLuidIndex[24..48],
    IfType set_IfType[48..64],
]}
UNION!{union NET_LUID_LH {
    [u64; 1],
    Value Value_mut: ULONG64,
    Info Info_mut: NET_LUID_LH_Info,
}}
pub type PNET_LUID_LH = *mut NET_LUID_LH;
pub type NET_IF_RCV_ADDRESS = NET_IF_RCV_ADDRESS_LH;
pub type PNET_IF_RCV_ADDRESS = *mut NET_IF_RCV_ADDRESS;
pub type NET_IF_ALIAS = NET_IF_ALIAS_LH;
pub type PNET_IF_ALIAS = *mut NET_IF_ALIAS;
pub type NET_LUID = NET_LUID_LH;
pub type PNET_LUID = *mut NET_LUID;
pub type IF_LUID = NET_LUID;
pub type PIF_LUID = *mut NET_LUID;
pub type NET_IFINDEX = ULONG;
pub type PNET_IFINDEX = *mut NET_IFINDEX;
pub type NET_IFTYPE = UINT16;
pub type PNET_IFTYPE = *mut NET_IFTYPE;
pub type IF_INDEX = NET_IFINDEX;
pub type PIF_INDEX = *mut NET_IFINDEX;
ENUM!{enum NET_IF_CONNECTION_TYPE {
    NET_IF_CONNECTION_DEDICATED = 1,
    NET_IF_CONNECTION_PASSIVE = 2,
    NET_IF_CONNECTION_DEMAND = 3,
    NET_IF_CONNECTION_MAXIMUM = 4,
}}
pub type PNET_IF_CONNECTION_TYPE = *mut NET_IF_CONNECTION_TYPE;
ENUM!{enum TUNNEL_TYPE {
    TUNNEL_TYPE_NONE = 0,
    TUNNEL_TYPE_OTHER = 1,
    TUNNEL_TYPE_DIRECT = 2,
    TUNNEL_TYPE_6TO4 = 11,
    TUNNEL_TYPE_ISATAP = 13,
    TUNNEL_TYPE_TEREDO = 14,
    TUNNEL_TYPE_IPHTTPS = 15,
}}
pub type PTUNNEL_TYPE = *mut TUNNEL_TYPE;
ENUM!{enum NET_IF_ACCESS_TYPE {
    NET_IF_ACCESS_LOOPBACK = 1,
    NET_IF_ACCESS_BROADCAST = 2,
    NET_IF_ACCESS_POINT_TO_POINT = 3,
    NET_IF_ACCESS_POINT_TO_MULTI_POINT = 4,
    NET_IF_ACCESS_MAXIMUM = 5,
}}
pub type PNET_IF_ACCESS_TYPE = *mut NET_IF_ACCESS_TYPE;
ENUM!{enum NET_IF_DIRECTION_TYPE {
    NET_IF_DIRECTION_SENDRECEIVE,
    NET_IF_DIRECTION_SENDONLY,
    NET_IF_DIRECTION_RECEIVEONLY,
    NET_IF_DIRECTION_MAXIMUM,
}}
pub type PNET_IF_DIRECTION_TYPE = *mut NET_IF_DIRECTION_TYPE;
ENUM!{enum NET_IF_MEDIA_CONNECT_STATE {
    MediaConnectStateUnknown,
    MediaConnectStateConnected,
    MediaConnectStateDisconnected,
}}
pub type PNET_IF_MEDIA_CONNECT_STATE = *mut NET_IF_MEDIA_CONNECT_STATE;
ENUM!{enum NET_IF_MEDIA_DUPLEX_STATE {
    MediaDuplexStateUnknown = 0,
    MediaDuplexStateHalf = 1,
    MediaDuplexStateFull = 2,
}}
pub type PNET_IF_MEDIA_DUPLEX_STATE = *mut NET_IF_MEDIA_DUPLEX_STATE;
STRUCT!{struct NET_PHYSICAL_LOCATION_LH {
    BusNumber: ULONG,
    SlotNumber: ULONG,
    FunctionNumber: ULONG,
}}
pub type PNET_PHYSICAL_LOCATION_LH = *mut NET_PHYSICAL_LOCATION_LH;
pub const IF_MAX_STRING_SIZE: usize = 256;
pub const IF_MAX_PHYS_ADDRESS_LENGTH: usize = 32;
STRUCT!{struct IF_COUNTED_STRING_LH {
    Length: USHORT,
    String: [WCHAR; IF_MAX_STRING_SIZE + 1],
}}
pub type PIF_COUNTED_STRING_LH = *mut IF_COUNTED_STRING_LH;
STRUCT!{struct IF_PHYSICAL_ADDRESS_LH {
    Length: USHORT,
    Address: [UCHAR; IF_MAX_PHYS_ADDRESS_LENGTH],
}}
pub type PIF_PHYSICAL_ADDRESS_LH = *mut IF_PHYSICAL_ADDRESS_LH;
pub type NET_PHYSICAL_LOCATION = NET_PHYSICAL_LOCATION_LH;
pub type PNET_PHYSICAL_LOCATION = *mut NET_PHYSICAL_LOCATION;
pub type IF_COUNTED_STRING = IF_COUNTED_STRING_LH;
pub type PIF_COUNTED_STRING = *mut IF_COUNTED_STRING;
pub type IF_PHYSICAL_ADDRESS = IF_PHYSICAL_ADDRESS_LH;
pub type PIF_PHYSICAL_ADDRESS = *mut IF_PHYSICAL_ADDRESS;
ENUM!{enum IF_ADMINISTRATIVE_STATE {
    IF_ADMINISTRATIVE_DISABLED = 0,
    IF_ADMINISTRATIVE_ENABLED = 1,
    IF_ADMINISTRATIVE_DEMANDDIAL = 2,
}}
pub type PIF_ADMINISTRATIVE_STATE = *mut IF_ADMINISTRATIVE_STATE;
ENUM!{enum IF_OPER_STATUS {
    IfOperStatusUp = 1,
    IfOperStatusDown,
    IfOperStatusTesting,
    IfOperStatusUnknown,
    IfOperStatusDormant,
    IfOperStatusNotPresent,
    IfOperStatusLowerLayerDown,
}}
STRUCT!{struct NDIS_INTERFACE_INFORMATION {
    ifOperStatus: NET_IF_OPER_STATUS,
    ifOperStatusFlags: ULONG,
    MediaConnectState: NET_IF_MEDIA_CONNECT_STATE,
    MediaDuplexState: NET_IF_MEDIA_DUPLEX_STATE,
    ifMtu: ULONG,
    ifPromiscuousMode: BOOLEAN,
    ifDeviceWakeUpEnable: BOOLEAN,
    XmitLinkSpeed: ULONG64,
    RcvLinkSpeed: ULONG64,
    ifLastChange: ULONG64,
    ifCounterDiscontinuityTime: ULONG64,
    ifInUnknownProtos: ULONG64,
    ifInDiscards: ULONG64,
    ifInErrors: ULONG64,
    ifHCInOctets: ULONG64,
    ifHCInUcastPkts: ULONG64,
    ifHCInMulticastPkts: ULONG64,
    ifHCInBroadcastPkts: ULONG64,
    ifHCOutOctets: ULONG64,
    ifHCOutUcastPkts: ULONG64,
    ifHCOutMulticastPkts: ULONG64,
    ifHCOutBroadcastPkts: ULONG64,
    ifOutErrors: ULONG64,
    ifOutDiscards: ULONG64,
    ifHCInUcastOctets: ULONG64,
    ifHCInMulticastOctets: ULONG64,
    ifHCInBroadcastOctets: ULONG64,
    ifHCOutUcastOctets: ULONG64,
    ifHCOutMulticastOctets: ULONG64,
    ifHCOutBroadcastOctets: ULONG64,
    CompartmentId: NET_IF_COMPARTMENT_ID,
    SupportedStatistics: ULONG,
}}
pub type PNDIS_INTERFACE_INFORMATION = *mut NDIS_INTERFACE_INFORMATION;