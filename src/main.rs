struct Product {
    id: u32,
    name: String,
    manufacturer: String,
    owner: String,
    status: String,
}

fn transfer_product(product: &mut Product, new_owner: String) {
    product.owner = new_owner;
    product.status = String::from("In Transit");
}

fn main() {
    let mut product = Product {
        id: 1,
        name: String::from("Organic Wheat"),
        manufacturer: String::from("ABC Farms"),
        owner: String::from("ABC Farms"),
        status: String::from("Manufactured"),
    };

    println!("Product ID: {}", product.id);
    println!("Product Name: {}", product.name);
    println!("Manufacturer: {}", product.manufacturer);
    println!("Current Owner: {}", product.owner);
    println!("Status: {}", product.status);

    transfer_product(&mut product, String::from("XYZ Distributor"));

    println!("After Transfer:");
    println!("Current Owner: {}", product.owner);
    println!("Status: {}", product.status);
}
