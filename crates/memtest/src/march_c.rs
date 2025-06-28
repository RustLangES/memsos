use core::ptr::{read_volatile, write_volatile};

pub fn march_c_test(buffer: *mut u8, end: usize) {
   for i in 0..=end {
       unsafe {  
            let ptr = buffer.add(i);
            if read_volatile(ptr) != 0 {
                // TODO: report error
                continue;
            }
            write_volatile(ptr, 1);
       }
   } 
   for i in (0..=end).rev() {
        unsafe {  
            let ptr = buffer.add(i);
            if read_volatile(ptr) != 1 {
                // TODO: report error
                continue;
            }
            write_volatile(ptr, 0);
       }
   }
      for i in (0..=end).rev() {
        unsafe {  
            let ptr = buffer.add(i);
            if read_volatile(ptr) != 0 {
                // TODO: report error
                continue;
            }
       }
   }

}


