
use diem_types::account_address::AccountAddress;
use uuid::Uuid;


#[test]
fn test_transfer_simple() {
    let escrow_addr = "708B1D23219EB737035CB29A68F0F3A8"
        .parse::<AccountAddress>().unwrap();
    let contract = bridge_ol::contract::BridgeEscrow::new(escrow_addr);
    assert!(contract.is_ok());
    let receiver_addr = hex::decode("15d34aaf54267db7d7c367839aaf71a00a2c6a65").unwrap();
    let transfer_id = Uuid::new_v4().as_bytes().to_vec();
    let res = contract
        .unwrap()
        .bridge_deposit(
        AccountAddress::ZERO,
        receiver_addr,
        10,
        transfer_id,
        None,
    );
    println!("{:?}",res);
    assert!(res.is_ok());
}