use std::io;

fn main(){
    money_convert();
}

fn money_convert() {
    println!("###############");
    println!("Money Conversion");
    println!("###############");
    println!("Enter the amount: ");

    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read input");
    let y: u32 = x.trim().parse().expect("Invalid input value");

    println!("Enter the current currency of the value:");
    let mut cur = String::new();
    io::stdin()
        .read_line(&mut cur)
        .expect("Failed to read the input");
    let c = cur.trim().to_uppercase();

    let mut a = String::new();
    println!("Enter the currency in which value would change : ");
    io::stdin().read_line(&mut a).expect("INVALID INPUT");
    let a = a.trim().to_uppercase();

    match c.as_str() {
        "USD" | "usd" => {
            if a == "INR" {
                println!("The converted amount is {}", y * 82)
            } else if a == "YEN" {
                println!("The converted amount is {}", y as f64 * 0.0064)
            } else if a == "EURO" {
                println!("The converted amount is {}", y as f64 * 1.09)
            } else if a == "POUND" {
                println!("The converted amount is {}", y as f64 * 1.29)
            } else if a == "FRANC" {
                println!("The converted amount is {}", y as f64 * 1.13)
            } else if a == "DINAR" {
                println!("The converted amount is {}", y as f64 * 0.000761)
            } else if a == "DIRHAM" {
                println!("The converted amount is {}", y as f64 * 0.27)
            } else if a == "RUBLES" {
                println!("The converted amount is {}", y as f64 * 0.011)
            } else {
                println!("No data available !")
            }
        }
        "INR" | "inr" => {
            if a == "USD" {
                println!("The converted amount is {}", y * 83)
            } else if a == "YEN" {
                println!("The converted amount is {}", y as f64 * 0.53)
            } else if a == "EURO" {
                println!("The converted amount is {}", y as f64 * 91.11)
            } else if a == "POUND" {
                println!("The converted amount is {}", y as f64 * 108.14)
            } else if a == "FRANC" {
                println!("The converted amount is {}", y as f64 * 94.19)
            } else if a == "DINAR" {
                println!("The converted amount is {}", y as f64 * 273.790511)
            } else if a == "DIRHAM" {
                println!("The converted amount is {}", y as f64 * 22.79)
            } else if a == "RUBLES" {
                println!("The converted amount is {}", y as f64 * 0.95)
            } else {
                println!("No data available !")
            }
        }
        "YEN" | "yen" => {
            if a == "USD" {
                println!("The converted amount is {}", y as f64 / 0.0063)
            } else if a == "INR" {
                println!("The converted amount is {}", y as f64 / 0.53)
            } else if a == "EURO" {
                println!("The converted amount is {}", y as f64 * 171.58)
            } else if a == "POUND" {
                println!("The converted amount is {}", y as f64 * 203.43)
            } else if a == "FRANC" {
                println!("The converted amount is {}", y as f64 * 177.19)
            } else if a == "DINAR" {
                println!("The converted amount is {}", y as f64 * 515.33)
            } else if a == "DIRHAM" {
                println!("The converted amount is {}", y as f64 * 42.88)
            } else if a == "RUBLES" {
                println!("The converted amount is {}", y as f64 * 1.79)
            } else {
                println!("No data available !")
            }
        }
        "EURO" | "euro" => {
            if a == "USD" {
                println!("The converted amount is {}", y as f64 * 0.92)
            } else if a == "INR" {
                println!("The converted amount is {}", y as f64 * 0.011)
            } else if a == "YEN" {
                println!("The converted amount is {}", y as f64 * 0.0058)
            } else if a == "POUND" {
                println!("The converted amount is {}", y as f64 * 1.19)
            } else if a == "FRANC" {
                println!("The converted amount is {}", y as f64 * 1.03)
            } else if a == "DINAR" {
                println!("The converted amount is {}", y as f64 * 3.004328)
            } else if a == "DIRHAM" {
                println!("The converted amount is {}", y as f64 * 0.25)
            } else if a == "RUBLES" {
                println!("The converted amount is {}", y as f64 * 0.01)
            } else {
                println!("No data available !")
            }
        }
        "POUND" | "pound" => {
            if a == "USD" {
                println!("The converted amount is {}", y as f64 * 0.77)
            } else if a == "INR" {
                println!("The converted amount is {}", y as f64 * 0.0092)
            } else if a == "YEN" {
                println!("The converted amount is {}", y as f64 * 0.0049)
            } else if a == "EURO" {
                println!("The converted amount is {}", y as f64 * 0.84)
            } else if a == "FRANC" {
                println!("The converted amount is {}", y as f64 * 0.87)
            } else if a == "DINAR" {
                println!("The converted amount is {}", y as f64 * 2.53)
            } else if a == "DIRHAM" {
                println!("The converted amount is {}", y as f64 * 0.21)
            } else if a == "RUBLES" {
                println!("The converted amount is {}", y as f64 * 0.0088)
            } else {
                println!("No data available !")
            }
        }
        "FRANC" | "franc" => {
            if a == "USD" {
                println!("The converted amount is {}", y as f64 * 0.89)
            } else if a == "INR" {
                println!("The converted amount is {}", y as f64 * 0.011)
            } else if a == "YEN" {
                println!("The converted amount is {}", y as f64 * 0.0056)
            } else if a == "POUND" {
                println!("The converted amount is {}", y as f64 * 1.15)
            } else if a == "EURO" {
                println!("The converted amount is {}", y as f64 * 0.97)
            } else if a == "DINAR" {
                println!("The converted amount is {}", y as f64 * 2.91)
            } else if a == "DIRHAM" {
                println!("The converted amount is {}", y as f64 * 0.24)
            } else if a == "RUBLES" {
                println!("The converted amount is {}", y as f64 * 0.01)
            } else {
                println!("No data available !")
            }
        }
        "DINAR" | "dinar" => {
            if a == "USD" {
                println!("The converted amount is {}", y as f64 * 0.305)
            } else if a == "INR" {
                println!("The converted amount is {}", y as f64 * 0.00365)
            } else if a == "YEN" {
                println!("The converted amount is {}", y as f64 * 0.20)
            } else if a == "POUND" {
                println!("The converted amount is {}", y as f64 * 4.00)
            } else if a == "FRANC" {
                println!("The converted amount is {}", y as f64 * 120.92)
            } else if a == "EURO" {
                println!("The converted amount is {}", y as f64 * 3.374)
            } else if a == "DIRHAM" {
                println!("The converted amount is {}", y as f64 * 0.08326)
            } else if a == "RUBLES" {
                println!("The converted amount is {}", y as f64 * 0.0035)
            } else {
                println!("No data available !")
            }
        }
        "DIRHAM" | "dirham" => {
            if a == "USD" {
                println!("The converted amount is {}", y as f64 * 3.67)
            } else if a == "INR" {
                println!("The converted amount is {}", y as f64 * 0.044)
            } else if a == "YEN" {
                println!("The converted amount is {}", y as f64 * 0.023)
            } else if a == "POUND" {
                println!("The converted amount is {}", y as f64 * 4.74)
            } else if a == "FRANC" {
                println!("The converted amount is {}", y as f64 * 4.13)
            } else if a == "DINAR" {
                println!("The converted amount is {}", y as f64 * 12.01)
            } else if a == "EURO" {
                println!("The converted amount is {}", y as f64 * 4.0)
            } else if a == "RUBLES" {
                println!("The converted amount is {}", y as f64 * 0.042)
            } else {
                println!("No data available !")
            }
        }
        "RUBLES" | "ruble" => {
            if a == "USD" {
                println!("The converted amount is {}", y as f64 * 87.87)
            } else if a == "INR" {
                println!("The converted amount is {}", y as f64 * 1.05)
            } else if a == "YEN" {
                println!("The converted amount is {}", y as f64 * 0.56)
            } else if a == "POUND" {
                println!("The converted amount is {}", y as f64 * 113.50)
            } else if a == "FRANC" {
                println!("The converted amount is {}", y as f64 * 98.86)
            } else if a == "DINAR" {
                println!("The converted amount is {}", y as f64 * 287.52)
            } else if a == "DIRHAM" {
                println!("The converted amount is {}", y as f64 * 23.92)
            } else if a == "EURO" {
                println!("The converted amount is {}", y as f64 * 95.97)
            } else {
                println!("No data available !")
            }
        }
        _ => println!("Dont know the currency"),
    }
}
