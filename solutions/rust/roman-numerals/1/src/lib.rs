use std::fmt::{Display, Formatter, Result};
use std::collections::BTreeMap;

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        // todo!("Return a roman-numeral string representation of the Roman object");
        write!(_f, "{}", self.0)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        // todo!("Construct a Roman object from the '{num}' number");

        let roman: BTreeMap<u32, &str> = BTreeMap::from([
                    (1000, "M"),
                    (900, "CM"),
                    (500, "D"),
                    (400, "CD"),
                    (100, "C"),
                    (90, "XC"),
                    (50, "L"),
                    (40, "XL"),
                    (10, "X"),
                    (9, "IX"),
                    (5, "V"),
                    (4, "IV"),
                    (1, "I"),
                ]);
        let mut result = String::new();
        let mut n: u32 = num;

        // Iterate in descending order (largest to smallest)
        for (&value, &symbol) in roman.iter().rev() {
            while n >= value {
                result.push_str(symbol);
                n -= value;
            }
        }

    Roman(result)
    }
}
    
    