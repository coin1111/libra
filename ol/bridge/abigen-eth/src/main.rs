use ethers::{ utils::Solc};
use ethers_contract_abigen::Abigen;
use std::process::exit;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    args.next().unwrap(); // skip program name
    let help = ||{
        println!("Usage: abigen-eth <contract-name> <contract-abi.json> [contract-abi.rs]");
        println!("Generate ETH contract abi for rust compiler");
        exit(0);
    };
    let contract_name =  args.next().unwrap_or_else(help);
    if &contract_name == "-h" || &contract_name == "--help" {
        help();
    }
    let contract = args.next().unwrap_or_else(help);



    println!("Generating bindings for {}\n", contract);

    // compile it if needed
    let abi = if contract.ends_with(".sol") {
        let contracts = Solc::new(&contract).build_raw()?;
        contracts.get(&contract_name).unwrap().abi.clone()
    } else {
        contract.clone()
    };

    let bindings = Abigen::new(&contract_name, abi)?.generate()?;

    // print to stdout if no output arg is given
    if let Some(output_path) = args.next() {
        bindings.write_to_file(&output_path)?;
    } else {
        bindings.write(std::io::stdout())?;
    }

    Ok(())
}