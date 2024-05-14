use std::cell::UnsafeCell;

fn opaque_read(val: &i32) {
    println!("{}", val);
}

fn main() {
    //unsafe {
    //    let mut data = 10;
    //    let mref1 = &mut data;
    //    let ptr2 = mref1 as *mut i32;
    //    let sref3 = &*mref1;
    //
    //    *ptr2 += 2;
    //    opaque_read(sref3); // Read in the wrong order?
    //    *mref1 += 1;
    //
    //    opaque_read(&data);
    //    println!();
    //}

    //unsafe {
    //    let mut data = Cell::new(10);
    //    let mref1 = &mut data;
    //    let ptr2 = mref1 as *mut Cell<i32>;
    //    let sref3 = &*mref1;
    //
    //    sref3.set(sref3.get() + 3);
    //    (*ptr2).set((*ptr2).get() + 2);
    //    mref1.set(mref1.get() + 1);
    //
    //    println!("{}", data.get());
    //}

    // unsafe {
    //     let mut data = UnsafeCell::new(10);
    //     let mref1 = data.get_mut();      // Get a mutable ref to the contents
    //     let ptr2 = mref1 as *mut i32;
    //     let sref3 = &*ptr2;
    //
    //     *ptr2 += 2;
    //     opaque_read(sref3);
    //     *mref1 += 1;
    //
    //     println!("{}", *data.get());
    // }

    //unsafe {
    //    let mut data = UnsafeCell::new(10);
    //    let mref1 = &mut data;              // Mutable ref to the *outside*
    //    let ptr2 = mref1.get();             // Get a raw pointer to the insides
    //    let sref3 = &*mref1;                // Get a shared ref to the *outside*
    //
    //    *ptr2 += 2;                         // Mutate with the raw pointer
    //    opaque_read(&*sref3.get());         // Read from the shared ref
    //    *sref3.get() += 3;                  // Write through the shared ref
    //    *mref1.get() += 1;                  // Mutate with the mutable ref
    //
    //    println!("{}", *data.get());
    //}

    //unsafe {
    //    let mut data = UnsafeCell::new(10);
    //    let mref1 = &mut data;
    //    // These two are swapped so the borrows are *definitely* totally stacked
    //    let sref2 = &*mref1;
    //    // Derive the ptr from the shared ref to be super safe!
    //    let ptr3 = sref2.get();
    //
    //    *ptr3 += 3;
    //    opaque_read(&*sref2.get());
    //    *sref2.get() += 2;
    //    *mref1.get() += 1;
    //
    //    println!("{}", *data.get());
    //}

    //unsafe {
    //    let mut data = Box::new(10);
    //    let ptr1 = (&mut *data) as *mut i32;
    //
    //    *data += 10;
    //    *ptr1 += 1;
    //
    //    // Should be 21
    //    println!("{}", data);
    //}

    unsafe {
        let mut data = Box::new(10);
        let ptr1 = (&mut *data) as *mut i32;

        *ptr1 += 1;
        *data += 10;

        // Should be 21
        println!("{}", data);
    }
}
