#[derive(Copy, Clone)]
struct MaybeUninhabited<const UNINHABITED: usize> {
    // ZST or uninhabited
    _field: [core::convert::Infallible; UNINHABITED],
}

impl<const UNINHABITED: usize> MaybeUninhabited<UNINHABITED> {
    const fn new() -> Self {
        const {
            if UNINHABITED == 0 {
                assert!(core::mem::size_of::<Self>() == 0);
                // Just going through &[].try_into().unwrap() doesn't cut it b/c that's still only
                // present for finitely many arrays
                //
                // This is through unsafe, but the assert above makes it pretty obvious that it is
                // safe here.
                unsafe { core::mem::zeroed() }
            } else {
                panic!("This is uninhabited")
            }
        }
    }
}

const fn guiding() -> usize {
    0
}

fn main() {
    println!("Hello");
    let _a: MaybeUninhabited<0> = MaybeUninhabited::new();
    const GUIDANCE: usize = guiding();
    let _a: MaybeUninhabited<GUIDANCE> = MaybeUninhabited::new();
    //let _b: MaybeUninhabited<1> = MaybeUninhabited::new();
}
