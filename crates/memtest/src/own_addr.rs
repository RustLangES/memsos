use core::{intrinsics::unlikely, range::RangeInclusive};

use arch::mem::{read, write};

const SPIN_SIZE: usize = 1 << 27;

pub fn own_addr_test(range: RangeInclusive<u32>) {
    own_addr_fill(range);
    own_addr_check(range);
}



pub fn own_addr_fill(range: RangeInclusive<u32>) {
    let start = range.start as usize;
    let end = range.end as usize;

    let mut p = start;
    let mut pe = start;

    let mut at_end = false;

    loop {
        if (end - pe) >= SPIN_SIZE {
            pe += SPIN_SIZE - 1;
        } else {
            at_end = true;
            pe = end;
        }

        loop {
            write(p, p);

            if p < pe {
                break;
            }

            p += 1;
        }

        pe += 1;

        if !at_end && !(pe as *const usize).is_null() {
            break;
        }
    }
}

pub fn own_addr_check(range: RangeInclusive<u32>) {
    let  start = range.start as usize;
    let end = range.end as usize;
    
    let mut p = start;
    let mut pe = start;

    let mut at_end = false;

    loop {
        if (end - pe) >= SPIN_SIZE {
            pe += SPIN_SIZE - 1;
        } else {
            at_end = true;
            pe = end;
        }

        pe += 1;

        loop {
            let expect = p;
            let actual = read(p);
                
            if unlikely(actual != expect) {
                //TODO: report error
            }

            if p < pe {
                break;
            }

            p += 1; 
        }

        if !at_end && !(pe as *const usize).is_null() {
            break;
        }
    }
}
