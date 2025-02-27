pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let fibs = fibnacci_less_than(threshold);

    fibs.iter().filter(|x| *x % 2 == 1).sum()
}


fn fibnacci_less_than(threshold: u32) -> Vec<u32>{
    let mut memos = Vec::new();
    memos.push(0);
    if threshold == 1 {
        return memos;
    }

    memos.push(1);
    let mut idx = 2;
    while memos[idx - 1] < threshold {
        let v = memos[idx -1] + memos[idx - 2];
        if v > threshold{
            break;
        }
        memos.push(v);
        idx += 1;
    }
    memos
}
