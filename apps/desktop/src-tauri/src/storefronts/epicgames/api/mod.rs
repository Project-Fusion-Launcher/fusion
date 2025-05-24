const USER_AGENT: &str =
    "UELauncher/11.0.1-14907503+++Portal+Release-Live Windows/10.0.19041.1.256.64bit";
const USER_BASIC: &str = "34a02cf8f4414e29b15921876da36f9a";
const PASSWORD_BASIC: &str = "daafbccc737745039dffe53d94fc76cf";

const ACCOUNT_HOST: &str = "https://account-public-service-prod03.ol.epicgames.com";
const LAUNCHER_HOST: &str = "https://launcher-public-service-prod06.ol.epicgames.com";
const CATALOG_HOST: &str = "https://catalog-public-service-prod06.ol.epicgames.com";

const MANIFEST_MAGIC: [u8; 4] = [0x0C, 0xC0, 0xBE, 0x44];
const CHUNK_MAGIC: [u8; 4] = [0xA2, 0x3A, 0xFE, 0xB1];

mod endpoints;
pub(super) mod models;
pub(super) mod services;
