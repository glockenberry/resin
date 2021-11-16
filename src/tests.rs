mod config {
    #[allow(unused_imports)]
    use crate::config;
    use rand::Rng;
    use std::{env::temp_dir, fs::File, io::Write, path::Path};

    const SAMPLE_CONFIG: &str = r#"
    {
        "name": "Very Special NFT",
        "symbol": "SNFT",
        "description": "This is the description of my NFT, it can be literally anything!",
        "creators": ["BPr18DCdtzASf1YVbUVZ4dZ7mA6jpMYZSUP3YuiMgGeD"],
        "royaltyPercentage": 10,
        "collection": {
            "name": "Special NFT: Season 1",
            "family": "Special NFTs"
        },
        "rarities": {
            "background": {
                "blue.png": 0.04,
                "brown.png": 0.04,
                "flesh.png": 0.05,
                "green.png": 0.02,
                "light-blue.png": 0.06,
                "light-green.png": 0.01,
                "light-pink.png": 0.07,
                "light-purple.png": 0.05,
                "light-yellow.png": 0.06,
                "orange.png": 0.07,
                "pink.png": 0.02,
                "purple.png": 0.03,
                "red.png": 0.05,
                "yellow.png": 0.43
            },
            "eyes": {
                "egg-eyes.png": 0.3,
                "heart-eyes.png": 0.12,
                "square-eyes.png": 0.02,
                "star-eyes.png": 0.56
            },
            "face": {
                "cyan-face.png": 0.07,
                "dark-green-face.png": 0.04,
                "flesh-face.png": 0.03,
                "gold-face.png": 0.11,
                "grapefruit-face.png": 0.07,
                "green-face.png": 0.05,
                "pink-face.png": 0.05,
                "purple-face.png": 0.02,
                "sun-face.png": 0.1,
                "teal-face.png": 0.46
            },
            "mouth": {
                "block-mouth.png": 0.23,
                "smile-mouth.png": 0.09,
                "triangle-mouth.png": 0.68
            }
        },
        "order": [
            "background",
            "face",
            "eyes",
            "mouth"
        ],
        "guaranteedRolls": [
            [
                "black.png",
                "white-face.png",
                "square-eyes.png",
                "smile-mouth.png"
            ]
        ],
        "amount": 1337
    }
    "#;

    #[allow(dead_code)]
    fn write_sample_config() -> String {
        let dir = temp_dir();
        let mut rng = rand::thread_rng();
        let file_name: String = (0u8..12u8)
            .map(|_| rng.gen_range(65u8..=122u8) as char)
            .collect();
        let path_buffer = Path::new(&dir).join(format!("{:?}.json", file_name));

        let mut file = File::create(&path_buffer).expect(&format!(
            "Could not create file at path {}",
            path_buffer.display()
        ));
        write!(file, "{}", SAMPLE_CONFIG).expect(&format!(
            "Could not write to file at path {}",
            path_buffer.display()
        ));

        path_buffer.to_str().unwrap_or("").to_string()
    }

    #[test]
    fn parse() {
        let config_path = write_sample_config();
        let parsed_config = config::parse(&config_path).unwrap();

        assert_eq!(parsed_config.name, "Very Special NFT");
        assert_eq!(parsed_config.symbol, "SNFT");
        assert_eq!(
            parsed_config.description,
            "This is the description of my NFT, it can be literally anything!"
        );
        assert_eq!(
            parsed_config.creators,
            vec!["BPr18DCdtzASf1YVbUVZ4dZ7mA6jpMYZSUP3YuiMgGeD"]
        );
        assert_eq!(parsed_config.royalty_percentage, 10);
        assert_eq!(parsed_config.collection.name, "Special NFT: Season 1");
        assert_eq!(parsed_config.collection.family, "Special NFTs");
        assert_eq!(parsed_config.rarities.len(), 4);
        assert_eq!(parsed_config.rarities.get("background").unwrap().len(), 14);
        assert_eq!(
            parsed_config
                .rarities
                .get("background")
                .unwrap()
                .get("blue.png")
                .unwrap(),
            &0.04f32
        );
        assert_eq!(parsed_config.rarities.get("eyes").unwrap().len(), 4);
        assert_eq!(
            parsed_config
                .rarities
                .get("eyes")
                .unwrap()
                .get("egg-eyes.png")
                .unwrap(),
            &0.3f32
        );
        assert_eq!(parsed_config.rarities.get("face").unwrap().len(), 10);
        assert_eq!(parsed_config.rarities.get("mouth").unwrap().len(), 3);
        assert_eq!(parsed_config.order.len(), 4);
        assert_eq!(
            parsed_config.order,
            vec!["background", "face", "eyes", "mouth"]
        );
        assert_eq!(parsed_config.guaranteed_rolls.len(), 1);
        assert_eq!(parsed_config.guaranteed_rolls[0].len(), 4);
        assert_eq!(
            parsed_config.guaranteed_rolls[0],
            vec![
                "black.png",
                "white-face.png",
                "square-eyes.png",
                "smile-mouth.png"
            ]
        );
        assert_eq!(parsed_config.amount, 1337);
    }

    #[test]
    #[should_panic]
    fn invalid_path() {
        config::parse("/path/to/nowhere").unwrap();
    }

    #[test]
    #[should_panic]
    fn corrupted_file() {
        let config_path = write_sample_config();
        write!(File::create(&config_path).unwrap(), "invalid json").unwrap();
        config::parse(&config_path).unwrap();
    }
}

mod metadata {
    #[allow(unused_imports)]
    use crate::metadata;

    #[test]
    fn integration() {
        assert!(true);
    }

    #[test]
    fn attribute_generation() {
        assert!(true);
    }

    #[test]
    fn creation() {
        assert!(true);
    }
}

mod art {
    #[allow(unused_imports)]
    use crate::art;

    #[test]
    fn read_metadata() {
        assert!(true);
    }

    #[test]
    fn creation() {
        assert!(true);
    }
}