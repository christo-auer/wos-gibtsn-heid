#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_enum_serde() {
        // Test all variants can be properly serialized and deserialized
        let locations = [
            Location::HSLaMensa,
            Location::HSLaCafeteria,
            Location::UniRMensa,
            Location::UniRGaestesaal,
            Location::UniRCafeteriaPT,
            Location::UniRCafeteriaChemie,
            Location::UniRCafeteriaSammel,
            Location::UniRCafeteriaSport,
            Location::OthRMensa,
            Location::OthRMensaAbend,
            Location::OthRMensaPruefening,
            Location::UniPMensa,
            Location::UniPCafeteriaNikolakloster,
            Location::ThDegMensa,
            Location::TcCham,
            Location::EcPfarrkirchen,
            Location::TumStraubing,
        ];

        for location in locations {
            let serialized = serde_json::to_string(&location).unwrap();
            let deserialized: Location = serde_json::from_str(&serialized).unwrap();
            assert_eq!(location, deserialized);
        }
    }

    #[test]
    fn test_location_to_id() {
        // Test happy path - common locations
        assert_eq!(location_to_id(&Location::HSLaMensa), "HS-LA");
        assert_eq!(location_to_id(&Location::UniRMensa), "UNI-R");
        assert_eq!(location_to_id(&Location::OthRMensa), "HS-R-tag");
        
        // Test locations with longer IDs
        assert_eq!(location_to_id(&Location::HSLaCafeteria), "HS-LA-Cafeteria");
        assert_eq!(location_to_id(&Location::UniRCafeteriaSammel), "Cafeteria-Sammelgebaeude");
        
        // Test locations with special characters in description but not in ID
        assert_eq!(location_to_id(&Location::UniRGaestesaal), "UNI-R-Gs");
        assert_eq!(location_to_id(&Location::OthRMensaPruefening), "Cafeteria-Pruefening");
    }

    #[test]
    fn test_value_enum_implementation() {
        // Test that Location implements ValueEnum correctly
        // This is a compile-time check since ValueEnum is used for clap's CLI parsing
        // We can test the string representation matches our expectations
        
        use clap::ValueEnum;
        
        // Check that to_possible_value() returns the expected values
        let values = Location::value_variants()
            .iter()
            .map(|v| v.to_possible_value().unwrap().get_name().to_string())
            .collect::<Vec<_>>();
            
        // Check that all variants are represented
        assert_eq!(values.len(), 17);
        
        // Check a few specific values
        assert!(values.contains(&"hs-la-mensa".to_string()));
        assert!(values.contains(&"uni-r-mensa".to_string()));
        assert!(values.contains(&"tum-straubing".to_string()));
    }

    #[test]
    fn test_serde_rename() {
        // Test that serde rename attributes work correctly
        let locations_and_expected_ids = [
            (Location::HSLaMensa, "\"HS-LA\""),
            (Location::HSLaCafeteria, "\"HS-LA-Cafeteria\""),
            (Location::UniRMensa, "\"UNI-R\""),
            (Location::UniRGaestesaal, "\"UNI-R-Gs\""),
            (Location::UniRCafeteriaPT, "\"Cafeteria-PT\""),
            (Location::UniRCafeteriaChemie, "\"Cafeteria-Chemie\""),
            (Location::UniRCafeteriaSammel, "\"Cafeteria-Sammelgebaeude\""),
            (Location::UniRCafeteriaSport, "\"Cafeteria-Sport\""),
            (Location::OthRMensa, "\"HS-R-tag\""),
            (Location::OthRMensaAbend, "\"HS-R-abend\""),
            (Location::OthRMensaPruefening, "\"Cafeteria-Pruefening\""),
            (Location::UniPMensa, "\"UNI-P\""),
            (Location::UniPCafeteriaNikolakloster, "\"Cafeteria-Nikolakloster\""),
            (Location::ThDegMensa, "\"HS-DEG\""),
            (Location::TcCham, "\"TH-DEG-Cham\""),
            (Location::EcPfarrkirchen, "\"HS-PAN\""),
            (Location::TumStraubing, "\"HS-SR\""),
        ];
        
        for (location, expected_json) in locations_and_expected_ids {
            let serialized = serde_json::to_string(&location).unwrap();
            assert_eq!(serialized, expected_json);
        }
    }

    #[test]
    fn test_serde_deserialize() {
        // Test deserialization from strings to Location enum
        let ids_and_expected_locations = [
            ("\"HS-LA\"", Location::HSLaMensa),
            ("\"HS-LA-Cafeteria\"", Location::HSLaCafeteria),
            ("\"UNI-R\"", Location::UniRMensa),
            ("\"UNI-R-Gs\"", Location::UniRGaestesaal),
            ("\"Cafeteria-PT\"", Location::UniRCafeteriaPT),
            ("\"Cafeteria-Chemie\"", Location::UniRCafeteriaChemie),
            ("\"Cafeteria-Sammelgebaeude\"", Location::UniRCafeteriaSammel),
            ("\"Cafeteria-Sport\"", Location::UniRCafeteriaSport),
            ("\"HS-R-tag\"", Location::OthRMensa),
            ("\"HS-R-abend\"", Location::OthRMensaAbend),
            ("\"Cafeteria-Pruefening\"", Location::OthRMensaPruefening),
            ("\"UNI-P\"", Location::UniPMensa),
            ("\"Cafeteria-Nikolakloster\"", Location::UniPCafeteriaNikolakloster),
            ("\"HS-DEG\"", Location::ThDegMensa),
            ("\"TH-DEG-Cham\"", Location::TcCham),
            ("\"HS-PAN\"", Location::EcPfarrkirchen),
            ("\"HS-SR\"", Location::TumStraubing),
        ];
        
        for (json, expected_location) in ids_and_expected_locations {
            let deserialized: Location = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, expected_location);
        }
    }

    #[test]
    fn test_error_handling() {
        // Test deserialization of invalid location strings
        let invalid_locations = [
            "\"INVALID\"",
            "\"HS-LAA\"",
            "\"uni-r\"",  // Case sensitive
            "\"\"",       // Empty string
            "42",         // Number instead of string
        ];
        
        for invalid_json in invalid_locations {
            let result: Result<Location, _> = serde_json::from_str(invalid_json);
            assert!(result.is_err(), "Expected error for invalid input: {}", invalid_json);
        }
    }
}