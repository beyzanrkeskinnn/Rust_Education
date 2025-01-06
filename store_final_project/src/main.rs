use std::collections::HashMap;
use std::io::{self, Write};
#[allow(dead_code)]
struct Product {
    name: String,
    description: String,
    price: f32,
    quantity: u32,
}

struct Inventory {
    products: HashMap<String, Product>,
}
#[allow(dead_code)]
impl Inventory {
    // Yeni envanter oluşturma
    fn new() -> Self {
        Inventory {
            products: HashMap::new(),
        }
    }

    // Ürün ekleme
    fn add_product(&mut self, product: Product) {
        self.products.insert(product.name.clone(), product);
    }

    // Ürün silme
    fn remove_product(&mut self, name: &str) {
        self.products.remove(name);
    }

    // Ürün güncelleme
    fn update_product(&mut self, name: &str, updated_product: Product) {
        if let Some(product) = self.products.get_mut(name) {
            *product = updated_product;
        }
    }

    // Ürünü bulma
    fn find_product(&self, name: &str) -> Option<&Product> {
        self.products.get(name)
    }

    // Envanteri yazdırma
    fn print_inventory(&self) {
        println!("\n--- Inventory ---");

        for (_, product) in &self.products {
            println!("Name: {}", product.name);
            println!("Description: {}", product.description);
            println!("Price: {}", product.price);
            println!("Quantity: {}", product.quantity);
            println!("------------------------");
        }
    }
}

// Satış yapısını tanımlar: Satılan ürün adı, miktarı ve satış fiyatı.
#[derive(Debug)]
struct Sale {
    product_name: String,
    quantity: u32,
    sale_price: f32,
}
#[allow(dead_code)]
impl Sale {
    // Satışın toplam değerini hesaplama
    fn total(&self) -> f32 {
        self.quantity as f32 * self.sale_price
    }
}

// Satın alma yapısını tanımlar: Satın alınan ürün adı, miktarı ve satın alma fiyatı.
#[derive(Debug)]
struct Purchase {
    product_name: String,
    quantity: u32,
    purchase_price: f32,
}
#[allow(dead_code)]
impl Purchase {
    // Satın alımın toplam maliyetini hesaplama
    fn total_cost(&self) -> f32 {
        self.quantity as f32 * self.purchase_price
    }
}

// Kullanıcı kimlik doğrulama fonksiyonu
fn authenticate(username: &str, password: &str) -> bool {
    username == "admin" && password == "password"
}

// Menü yazdırma fonksiyonu
fn print_menu() {
    println!("\n--- Rusty Store Menu ---");
    println!("1. Add Product");
    println!("2. View Inventory");
    println!("3. Record Sale");
    println!("4. Record Purchase");
    println!("5. Exit");
}

fn main() {
    let mut inventory = Inventory::new(); // Envanteri başlat
    let mut sales: Vec<Sale> = Vec::new(); // Satışları tutacak vektör
    let mut purchases: Vec<Purchase> = Vec::new(); // Satın alımları tutacak vektör

    // Kullanıcı kimlik doğrulama işlemi
    let mut username = String::new();
    let mut password = String::new();

    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();

    // Kimlik doğrulama başarısızsa programdan çık
    if !authenticate(username.trim(), password.trim()) {
        println!("Authentication failed! Exiting...");
        return;
    }

    loop {
        print_menu(); // Menüyü yazdır
        let mut choice = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                // Ürün ekleme işlemi
                let mut name = String::new();
                let mut description = String::new();
                let mut price = String::new();
                let mut quantity = String::new();

                print!("Enter product name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut name).unwrap();

                print!("Enter product description: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut description).unwrap();

                print!("Enter product price: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut price).unwrap();

                print!("Enter product quantity: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut quantity).unwrap();

                let price: f32 = price.trim().parse().unwrap();
                let quantity: u32 = quantity.trim().parse().unwrap();

                let product = Product {
                    name: name.trim().to_string(),
                    description: description.trim().to_string(),
                    price,
                    quantity,
                };

                inventory.add_product(product);
                println!("Product added successfully!");
            }
            "2" => {
                // Envanteri görüntüleme
                inventory.print_inventory();
            }
            "3" => {
                // Satış kaydetme işlemi
                let mut product_name = String::new();
                let mut quantity_sold = String::new();
                let mut sale_price = String::new();

                print!("Enter product name for sale: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut product_name).unwrap();

                print!("Enter quantity sold: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut quantity_sold).unwrap();

                print!("Enter sale price: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut sale_price).unwrap();

                let quantity_sold: u32 = quantity_sold.trim().parse().unwrap();
                let sale_price: f32 = sale_price.trim().parse().unwrap();

                if let Some(product) = inventory.find_product(&product_name.trim()) {
                    if product.quantity >= quantity_sold {
                        let sale = Sale {
                            product_name: product_name.trim().to_string(),
                            quantity: quantity_sold,
                            sale_price,
                        };
                        sales.push(sale);
                        println!("Sale recorded successfully!");
                    } else {
                        println!("Not enough stock for this product!");
                    }
                } else {
                    println!("Product not found in inventory!");
                }
            }
            "4" => {
                // Satın alma kaydetme işlemi
                let mut product_name = String::new();
                let mut quantity_purchased = String::new();
                let mut purchase_price = String::new();

                print!("Enter product name for purchase: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut product_name).unwrap();

                print!("Enter quantity purchased: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut quantity_purchased).unwrap();

                print!("Enter purchase price: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut purchase_price).unwrap();

                let quantity_purchased: u32 = quantity_purchased.trim().parse().unwrap();
                let purchase_price: f32 = purchase_price.trim().parse().unwrap();

                let purchase = Purchase {
                    product_name: product_name.trim().to_string(),
                    quantity: quantity_purchased,
                    purchase_price,
                };

                purchases.push(purchase);
                println!("Purchase recorded successfully!");
            }
            "5" => {
                // Programdan çıkış
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}
