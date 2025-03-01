pub fn goldbach_conjecture() -> String {
    let mut primes: Vec<u64> =  Vec::new();
    let mut result: Vec<String> = Vec::new();
    for i in 2..10000u64{
        if is_prime(i) {
            // println!("Prime {}", i);
            primes.push(i);
            continue;
        }

        if is_odd(i){ // odd composite number
            let mut flag: bool = false;
            for p in &primes{
                let v = i - p;
                if !is_odd(v) && is_square(v / 2) {
                    flag = true;
                    break;
                }
            }

            if !flag {
                result.push(i.to_string());
            }
        }

        if result.len() == 2 {
            break;
        }
    }

    result.join(",")
}

fn is_odd(n: u64) -> bool{
    return n % 2 == 1;
}

fn is_square(n: u64) -> bool {
    let root = (n as f64).sqrt() as u64;
    root * root == n
}

fn is_prime(n: u64) -> bool{
    if n <= 1 {
        return false;
    }

    if n <= 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}