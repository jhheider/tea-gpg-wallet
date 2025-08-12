const GPG_DEPLOYER_ADDRESS: &str = "0x1e67d22542bd2eAFff45BEA53BebDA73E7A231dd";
const RPC_URL: &str = "https://tea-sepolia.g.alchemy.com/public";

fn main() {
    println!("cargo::rustc-env=GPG_DEPLOYER_ADDRESS={GPG_DEPLOYER_ADDRESS}");
    println!("cargo::rustc-env=RPC_URL={RPC_URL}");
}
