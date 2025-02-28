use std::collections::HashMap;

pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    if amount == 0 {
        return 0;
    }

    let mut memo  = HashMap::new();
    memo.insert(0, 0);
    dp_helper(amount,&mut memo)
}


fn dp_helper(amount: u32, memo:&mut HashMap<u32,u32>) -> u32{
    if let Some(&result) = memo.get(&amount){
        // println!("{} {}", amount, result);
        return result;
    }

    const CHANGES: [u32;8] = [1,2,5,10,20,30,50,100];
    
    let mut min_count = u32::MAX;
    for coin in CHANGES {
        if coin <= amount{
            let count = dp_helper(amount - coin, memo) + 1;
            min_count = min_count.min(count);
        }
    }
    memo.insert(amount, min_count);

    min_count
}