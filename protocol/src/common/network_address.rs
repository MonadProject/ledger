use common::ip_address::IpAddress;
use common::services::Services;

// see https://en.bitcoin.it/wiki/Protocol_documentation#Network_address
pub struct Network_Address {
    services: Services,
    ipAddress: IpAddress,
    port: u16,
}