use std::io;

#[derive(Debug, Clone)]
enum ProductStatus {
    Manufactured,
    InTransit,
    AtWarehouse,
    Delivered,
}

impl ProductStatus {
    fn display(&self) -> &str {
        match self {
            ProductStatus::Manufactured => "Manufactured",
            ProductStatus::InTransit => "In Transit",
            ProductStatus::AtWarehouse => "At Warehouse",
            ProductStatus::Delivered => "Delivered",
        }
    }
}

struct HistoryRecord {
    owner: String,
    status: ProductStatus,
}

struct Product {
    id: u32,
    name: String,
    manufacturer: String,
    owner: String,
    status: ProductStatus,
    history: Vec<HistoryRecord>,
}

fn create_product(
    id: u32,
    name: String,
    manufacturer: String,
) -> Product {
    let owner = manufacturer.clone();

    let history = vec![
        HistoryRecord {
            owner: owner.clone(),
            status: ProductStatus::Manufactured,
        }
    ];

    Product {
        id,
        name,
        manufacturer,
        owner,
        status: ProductStatus::Manufactured,
        history,
    }
}

fn find_product_mut(
    products: &mut Vec<Product>,
    product_id: u32,
) -> Option<&mut Product> {
    for product in products {
        if product.id == product_id {
            return Some(product);
        }
    }

    None
}

fn transfer_product(
    product: &mut Product,
    new_owner: String,
) {
    product.owner = new_owner.clone();
    product.status = ProductStatus::InTransit;

    product.history.push(
        HistoryRecord {
            owner: new_owner,
            status: ProductStatus::InTransit,
        }
    );
}

fn update_status(
    product: &mut Product,
    new_status: ProductStatus,
) {
    product.status = new_status.clone();

    product.history.push(
        HistoryRecord {
            owner: product.owner.clone(),
            status: new_status,
        }
    );
}

fn show_product(product: &Product) {
    println!();
    println!("=== Product Details ===");
    println!("Product ID: {}", product.id);
    println!("Product Name: {}", product.name);
    println!("Manufacturer: {}", product.manufacturer);
    println!("Current Owner: {}", product.owner);
    println!("Status: {}", product.status.display());
}

fn show_history(product: &Product) {
    println!();
    println!("=== Product History ===");

    for (index, record) in product.history.iter().enumerate() {
        println!(
            "{}. Owner: {} | Status: {}",
            index + 1,
            record.owner,
            record.status.display()
        );
    }
}

fn main() {
    println!("=== Blockchain Supply Chain Platform ===");

    let mut products: Vec<Product> = Vec::new();

    // Product 1
    println!();
    println!("Enter Product 1 Name:");

    let mut product_name = String::new();

    io::stdin()
        .read_line(&mut product_name)
        .expect("Failed to read input");

    let product_name = product_name.trim().to_string();

    println!("Enter Product 1 Manufacturer:");

    let mut manufacturer = String::new();

    io::stdin()
        .read_line(&mut manufacturer)
        .expect("Failed to read input");

    let manufacturer = manufacturer.trim().to_string();

    products.push(
        create_product(
            1,
            product_name,
            manufacturer,
        )
    );

    // Product 2
    println!();
    println!("Enter Product 2 Name:");

    let mut product_name = String::new();

    io::stdin()
        .read_line(&mut product_name)
        .expect("Failed to read input");

    let product_name = product_name.trim().to_string();

    println!("Enter Product 2 Manufacturer:");

    let mut manufacturer = String::new();

    io::stdin()
        .read_line(&mut manufacturer)
        .expect("Failed to read input");

    let manufacturer = manufacturer.trim().to_string();

    products.push(
        create_product(
            2,
            product_name,
            manufacturer,
        )
    );

    // Transfer Product
    println!();
    println!("Enter Product ID to Transfer:");

    let mut product_id = String::new();

    io::stdin()
        .read_line(&mut product_id)
        .expect("Failed to read input");

    let product_id: u32 = product_id
        .trim()
        .parse()
        .expect("Please enter a valid number");

    println!("Enter New Owner:");

    let mut new_owner = String::new();

    io::stdin()
        .read_line(&mut new_owner)
        .expect("Failed to read input");

    let new_owner = new_owner.trim().to_string();

    match find_product_mut(
        &mut products,
        product_id,
    ) {
        Some(product) => {
            transfer_product(
                product,
                new_owner,
            );

            println!();
            println!("Product Transferred Successfully!");

            show_product(product);
        }

        None => {
            println!("Product not found!");
            return;
        }
    }

    // At Warehouse
    match find_product_mut(
        &mut products,
        product_id,
    ) {
        Some(product) => {
            update_status(
                product,
                ProductStatus::AtWarehouse,
            );

            println!();
            println!("Product Reached Warehouse!");

            show_product(product);
        }

        None => {
            println!("Product not found!");
            return;
        }
    }

    // Delivered
    match find_product_mut(
        &mut products,
        product_id,
    ) {
        Some(product) => {
            update_status(
                product,
                ProductStatus::Delivered,
            );

            println!();
            println!("Product Delivered Successfully!");

            show_product(product);

            show_history(product);
        }

        None => {
            println!("Product not found!");
        }
    }
}