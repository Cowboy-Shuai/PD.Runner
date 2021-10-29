#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_NetworkManagement_Dhcp")]
pub mod Dhcp;
#[cfg(feature = "Win32_NetworkManagement_Dns")]
pub mod Dns;
#[cfg(feature = "Win32_NetworkManagement_InternetConnectionWizard")]
pub mod InternetConnectionWizard;
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
pub mod IpHelper;
#[cfg(feature = "Win32_NetworkManagement_MobileBroadband")]
pub mod MobileBroadband;
#[cfg(feature = "Win32_NetworkManagement_Multicast")]
pub mod Multicast;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub mod Ndis;
#[cfg(feature = "Win32_NetworkManagement_NetBios")]
pub mod NetBios;
#[cfg(feature = "Win32_NetworkManagement_NetManagement")]
pub mod NetManagement;
#[cfg(feature = "Win32_NetworkManagement_NetShell")]
pub mod NetShell;
#[cfg(feature = "Win32_NetworkManagement_NetworkDiagnosticsFramework")]
pub mod NetworkDiagnosticsFramework;
#[cfg(feature = "Win32_NetworkManagement_NetworkPolicyServer")]
pub mod NetworkPolicyServer;
#[cfg(feature = "Win32_NetworkManagement_P2P")]
pub mod P2P;
#[cfg(feature = "Win32_NetworkManagement_QoS")]
pub mod QoS;
#[cfg(feature = "Win32_NetworkManagement_Rras")]
pub mod Rras;
#[cfg(feature = "Win32_NetworkManagement_Snmp")]
pub mod Snmp;
#[cfg(feature = "Win32_NetworkManagement_WNet")]
pub mod WNet;
#[cfg(feature = "Win32_NetworkManagement_WebDav")]
pub mod WebDav;
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
pub mod WiFi;
#[cfg(feature = "Win32_NetworkManagement_WindowsConnectNow")]
pub mod WindowsConnectNow;
#[cfg(feature = "Win32_NetworkManagement_WindowsConnectionManager")]
pub mod WindowsConnectionManager;
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
pub mod WindowsFilteringPlatform;
#[cfg(feature = "Win32_NetworkManagement_WindowsFirewall")]
pub mod WindowsFirewall;
#[cfg(feature = "Win32_NetworkManagement_WindowsNetworkVirtualization")]
pub mod WindowsNetworkVirtualization;
