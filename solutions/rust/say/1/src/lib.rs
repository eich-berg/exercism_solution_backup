pub fn encode(n: u64) -> String {
    // todo!("Say {n} in English.");
    let div_100 = Vec::from(["zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy",     "eighty", "ninety"]);
    
    let div_10 = Vec::from(["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"]);
    
    if n < 20 { return div_10[n as usize].to_string(); }
    else if n < 100 {
        let div = n / 10;  
        let rest = n % 10; 
        if rest > 0 { return format!("{}-{}", div_100[div as usize], encode(rest)); } 
        else { return div_100[div as usize].to_string(); }
    }
    else if n < 1000 {
        let div = n / 100;  
        let rest = n % 100; 
        if rest > 0 { return format!("{} hundred {}", encode(div), encode(rest)); } 
        else { return encode(div) + " hundred"; }
    }
    else if n < 1_000_000 {
        let div = n / 1000;  
        let rest = n % 1000; 
        if rest > 0 { return format!("{} thousand {}", encode(div), encode(rest)); } 
        else { return encode(div) + " thousand"; }
    }
    else if n < 1_000_000_000 {
        let div = n / 1_000_000;  
        let rest = n % 1_000_000; 
        if rest > 0 { return format!("{} million {}", encode(div), encode(rest)); } 
        else { return encode(div) + " million"; }
    }
    else if n < 1_000_000_000_000 {
        let div = n / 1_000_000_000;  
        let rest = n % 1_000_000_000; 
        if rest > 0 { return format!("{} billion {}", encode(div), encode(rest)); } 
        else { return encode(div) + " billion"; }
    }
    else if n < 1_000_000_000_000_000 {
        let div = n / 1_000_000_000_000;  
        let rest = n % 1_000_000_000_000; 
        if rest > 0 { return format!("{} trillion {}", encode(div), encode(rest)); } 
        else { return encode(div) + " trillion"; }
    }
    else if n < 1_000_000_000_000_000_000 {
        let div = n / 1_000_000_000_000_000;  
        let rest = n % 1_000_000_000_000_000; 
        if rest > 0 { return format!("{} quadrillion {}", encode(div), encode(rest)); } 
        else { return encode(div) + " quadrillion"; }
    }
    else {
        let div = n / 1_000_000_000_000_000_000;  
        let rest = n % 1_000_000_000_000_000_000; 
        if rest > 0 { return format!("{} quintillion {}", encode(div), encode(rest)); } 
        else { return encode(div) + " quintillion"; }
    }
}