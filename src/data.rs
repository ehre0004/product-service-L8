use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Simple Monitor".to_string(),
            price: 9.99,
            description: "A simple monitor of average size and decent price.".to_string(),
            image: "/monitor.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Simple Mouse".to_string(),
            price: 6.99,
            description: "A simple mouse of average size and decent price.".to_string(),
            image: "/mouse.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Simple HDMI Cord".to_string(),
            price: 12.99,
            description: "A simple HDMI cord of average length and decent price.".to_string(),
            image: "/hdmi.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Simple USB Cord".to_string(),
            price: 11.99,
            description: "A simple USB cord of average length and decent price.".to_string(),
            image: "/usbcord.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Simple USB Stick".to_string(),
            price: 11.99,
            description: "A simple USB stick of average size and decent price.".to_string(),
            image: "/usbstick.jpg".to_string()
        }
    ]
}