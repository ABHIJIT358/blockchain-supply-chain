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

struct OwnershipRecord {
    owner: String,
    status: ProductStatus,
}

struct Product {
    id: u32,
    name: String,
    manufacturer: String,
    owner: String,
    status: ProductStatus,
    history: Vec<OwnershipRecord>,
}

fn transfer_product(product: &mut Product, new_owner: String) {
    product.owner = new_owner.clone();
    product.status = ProductStatus::InTransit;

    product.history.push(OwnershipRecord {
        owner: new_owner,
        status: ProductStatus::InTransit,
    });
}

fn update_status(product: &mut Product, new_status: ProductStatus) {
    product.status = new_status.clone();

    product.history.push(OwnershipRecord {
        owner: product.owner.clone(),
        status: new_status,
    });
}

fn find_product_mut(products: &mut Vec<Product>, product_id: u32) -> Option<&mut Product> {
    for product in products {
        if product.id == product_id {
            return Some(product);
        }
    }

    None
}

fn show_history(product: &Product) {
    println!();
    println!("Ownership / Status History:");

    for (index, record) in product.history.iter().enumerate() {
        println!(
            "{}. {} → {}",
            index + 1,
            record.owner,
            record.status.display()
        );
    }
}

fn main() {
    let mut products: Vec<Product> = Vec::new();

    let product1 = Product {
        id: 1,
        name: String::from("Organic Wheat"),
        manufacturer: String::from("ABC Farms"),
        owner: String::from("ABC Farms"),
        status: ProductStatus::Manufactured,
        history: vec![OwnershipRecord {
            owner: String::from("ABC Farms"),
            status: ProductStatus::Manufactured,
        }],
    };

    let product2 = Product {
        id: 2,
        name: String::from("Basmati Rice"),
        manufacturer: String::from("XYZ Farms"),
        owner: String::from("XYZ Farms"),
        status: ProductStatus::Manufactured,
        history: vec![OwnershipRecord {
            owner: String::from("XYZ Farms"),
            status: ProductStatus::Manufactured,
        }],
    };

    products.push(product1);
    products.push(product2);

    let product_id = 2;

    match find_product_mut(&mut products, product_id) {
        Some(product) => {
            transfer_product(product, String::from("Nagpur Warehouse"));

            update_status(product, ProductStatus::AtWarehouse);

            update_status(product, ProductStatus::Delivered);
        }

        None => {
            println!("Product not found!");
        }
    }

    println!("Product Status:");

    for product in &products {
        println!(
            "Product {} | {} | Owner: {} | Status: {}",
            product.id,
            product.name,
            product.owner,
            product.status.display()
        );
    }

    if let Some(product) = products.iter().find(|p| p.id == 2) {
        show_history(product);
    }
}
