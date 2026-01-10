// Currency units for LuxTensor
// 
// LuxTensor uses a simple unit system:
// - LTS (LuxTensor Smallest) is the base unit (smallest indivisible unit)
// - MDT (ModernTensor) is the main denomination
// - 1 MDT = 10^18 LTS (similar to ETH/wei relationship)
//
// This allows for precise fractional amounts while maintaining compatibility
// with the existing 18-decimal system used throughout the blockchain.

/// LTS: The smallest unit of currency in LuxTensor (base unit)
/// All balances are stored internally as LTS
pub const LTS_PER_MDT: u128 = 1_000_000_000_000_000_000; // 10^18

/// Helper constant for common denominations
pub const LTS_PER_KMDT: u128 = 1_000_000_000_000_000_000_000; // 10^21 (1000 MDT)
pub const LTS_PER_MMDT: u128 = 1_000_000_000_000_000_000_000_000; // 10^24 (1M MDT)

/// Number of decimal places for MDT
pub const MDT_DECIMALS: u8 = 18;

/// Convert MDT to LTS (smallest unit)
/// 
/// # Example
/// ```
/// use luxtensor_core::currency::mdt_to_lts;
/// 
/// let amount_mdt = 1;
/// let amount_lts = mdt_to_lts(amount_mdt);
/// assert_eq!(amount_lts, 1_000_000_000_000_000_000);
/// ```
pub fn mdt_to_lts(mdt: u128) -> u128 {
    mdt.saturating_mul(LTS_PER_MDT)
}

/// Convert LTS to MDT (with precision loss for fractional amounts)
/// 
/// # Example
/// ```
/// use luxtensor_core::currency::lts_to_mdt;
/// 
/// let amount_lts = 1_000_000_000_000_000_000;
/// let amount_mdt = lts_to_mdt(amount_lts);
/// assert_eq!(amount_mdt, 1);
/// ```
pub fn lts_to_mdt(lts: u128) -> u128 {
    lts / LTS_PER_MDT
}

/// Format LTS amount as a human-readable MDT string with decimals
/// 
/// # Example
/// ```
/// use luxtensor_core::currency::format_lts_as_mdt;
/// 
/// let amount_lts = 1_500_000_000_000_000_000;
/// let formatted = format_lts_as_mdt(amount_lts);
/// assert_eq!(formatted, "1.500000000000000000 MDT");
/// ```
pub fn format_lts_as_mdt(lts: u128) -> String {
    let mdt_whole = lts / LTS_PER_MDT;
    let lts_fractional = lts % LTS_PER_MDT;
    format!("{}.{:018} MDT", mdt_whole, lts_fractional)
}

/// Format LTS amount as raw LTS string
/// 
/// # Example
/// ```
/// use luxtensor_core::currency::format_lts;
/// 
/// let amount_lts = 1_500_000_000_000_000_000;
/// let formatted = format_lts(amount_lts);
/// assert_eq!(formatted, "1500000000000000000 LTS");
/// ```
pub fn format_lts(lts: u128) -> String {
    format!("{} LTS", lts)
}

/// Parse MDT string to LTS
/// Accepts formats like "1.5", "1", "0.5"
/// 
/// # Example
/// ```
/// use luxtensor_core::currency::parse_mdt_to_lts;
/// 
/// let amount_lts = parse_mdt_to_lts("1.5").unwrap();
/// assert_eq!(amount_lts, 1_500_000_000_000_000_000);
/// ```
pub fn parse_mdt_to_lts(mdt_str: &str) -> Result<u128, String> {
    let parts: Vec<&str> = mdt_str.split('.').collect();
    
    match parts.len() {
        1 => {
            // Whole number only
            let whole = parts[0].parse::<u128>()
                .map_err(|_| "Invalid MDT amount".to_string())?;
            Ok(mdt_to_lts(whole))
        }
        2 => {
            // Whole and fractional parts
            let whole = parts[0].parse::<u128>()
                .map_err(|_| "Invalid MDT amount".to_string())?;
            
            // Pad fractional part to 18 digits
            let mut frac_str = parts[1].to_string();
            if frac_str.len() > 18 {
                return Err("Too many decimal places (max 18)".to_string());
            }
            while frac_str.len() < 18 {
                frac_str.push('0');
            }
            
            let fractional = frac_str.parse::<u128>()
                .map_err(|_| "Invalid fractional amount".to_string())?;
            
            Ok(mdt_to_lts(whole).saturating_add(fractional))
        }
        _ => Err("Invalid MDT format".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mdt_to_lts() {
        assert_eq!(mdt_to_lts(0), 0);
        assert_eq!(mdt_to_lts(1), 1_000_000_000_000_000_000);
        assert_eq!(mdt_to_lts(10), 10_000_000_000_000_000_000);
        assert_eq!(mdt_to_lts(100), 100_000_000_000_000_000_000);
    }

    #[test]
    fn test_lts_to_mdt() {
        assert_eq!(lts_to_mdt(0), 0);
        assert_eq!(lts_to_mdt(1_000_000_000_000_000_000), 1);
        assert_eq!(lts_to_mdt(10_000_000_000_000_000_000), 10);
        assert_eq!(lts_to_mdt(1_500_000_000_000_000_000), 1); // Truncates fractional
    }

    #[test]
    fn test_format_lts_as_mdt() {
        assert_eq!(
            format_lts_as_mdt(1_000_000_000_000_000_000),
            "1.000000000000000000 MDT"
        );
        assert_eq!(
            format_lts_as_mdt(1_500_000_000_000_000_000),
            "1.500000000000000000 MDT"
        );
        assert_eq!(
            format_lts_as_mdt(0),
            "0.000000000000000000 MDT"
        );
    }

    #[test]
    fn test_format_lts() {
        assert_eq!(format_lts(1_000_000_000_000_000_000), "1000000000000000000 LTS");
        assert_eq!(format_lts(0), "0 LTS");
    }

    #[test]
    fn test_parse_mdt_to_lts() {
        assert_eq!(parse_mdt_to_lts("1").unwrap(), 1_000_000_000_000_000_000);
        assert_eq!(parse_mdt_to_lts("1.5").unwrap(), 1_500_000_000_000_000_000);
        assert_eq!(parse_mdt_to_lts("0.5").unwrap(), 500_000_000_000_000_000);
        assert_eq!(parse_mdt_to_lts("10").unwrap(), 10_000_000_000_000_000_000);
        
        // Test with full precision
        assert_eq!(
            parse_mdt_to_lts("1.000000000000000001").unwrap(),
            1_000_000_000_000_000_001
        );
        
        // Test error cases
        assert!(parse_mdt_to_lts("invalid").is_err());
        assert!(parse_mdt_to_lts("1.1.1").is_err());
        assert!(parse_mdt_to_lts("1.1234567890123456789").is_err()); // Too many decimals
    }

    #[test]
    fn test_constants() {
        assert_eq!(LTS_PER_MDT, 1_000_000_000_000_000_000);
        assert_eq!(LTS_PER_KMDT, 1_000_000_000_000_000_000_000);
        assert_eq!(LTS_PER_MMDT, 1_000_000_000_000_000_000_000_000);
        assert_eq!(MDT_DECIMALS, 18);
    }
}
