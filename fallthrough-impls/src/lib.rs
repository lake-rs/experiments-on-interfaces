/// Do our interfaces become overly complex if we allow composition from different backends?
///
/// (Otherwise, we'd just do the formal verification in provided functions, but that'd weaken the
/// point of being just like `embedded-[hn]al`).

pub trait UserFacing {
    // Those are more for detecting what an implementation can do
    type A: Copy + Clone;
    type B: Copy + Clone;
    fn can_a(&mut self) -> Option<Self::A>;
    fn can_b(&mut self) -> Option<Self::B>;

    // Those do actual work

    fn composite_function(&mut self, because_a: Self::A, because_b: Self::B);

    fn piece_a(&mut self, because: Self::A);
    fn piece_b(&mut self, because: Self::B);
}

pub struct Limited;

impl UserFacing for Limited {
    type A = ();
    // aka Never: we can't do it
    type B = core::convert::Infallible;

    fn can_a(&mut self) -> Option<Self::A> {
        Some(())
    }

    // This we can do:
    fn piece_a(&mut self, _because: Self::A) {
        println!("Performing function A in hardware");
    }

    // Those all collapse into nothingness -- It Is Known we can't do this.
    fn can_b(&mut self) -> Option<Self::B> {
        // This happens to be the only inhabitant of the ZST
        None
    }
    fn composite_function(&mut self, _because_a: Self::A, because_b: Self::B) {
        match because_b {}
    }
    fn piece_b(&mut self, because: Self::B) {
        match because {}
    }
}

pub struct SoftwareImplementation;

impl UserFacing for SoftwareImplementation {
    // We can do everything (but slowly)
    type A = ();
    type B = ();
    fn can_a(&mut self) -> Option<Self::A> {
        Some(())
    }
    fn can_b(&mut self) -> Option<Self::B> {
        Some(())
    }

    fn composite_function(&mut self, because_a: Self::A, because_b: Self::B) {
        // This is not ideal: If we're the fallback, we want to run either through the other.
        //
        // (Like, we can implement it here this way, but if this is used as a fallback, the
        // fallback will always overrule us)
        self.piece_a(because_a);
        self.piece_b(because_b);
    }

    fn piece_a(&mut self, _because: Self::A) {
        println!("Performing function A in software");
    }

    fn piece_b(&mut self, _because: Self::B) {
        println!("Performing function B in software");
    }
}

pub struct FallingBack<I1: UserFacing, I2: UserFacing> {
    main: I1,
    backup: I2,
}

#[derive(Copy, Clone)]
pub enum Either<I1, I2> {
    Main(I1),
    Backup(I2),
}

// Sadly we can't reuse this in the software impl because it takes 2 mut, but it's just two lines
// -- for an impl that is more complex, we may want to have an internal generic impl that has a
// .get_a_mut() and .get_b_mut() that can either take a FallingBack forward, or reverse, or just a
// &mut impl UserFacing.
fn run_fallback<IA: UserFacing, IB: UserFacing>(a: &mut IA, because_a: IA::A, b: &mut IB, because_b: IB::B) {
    a.piece_a(because_a);
    b.piece_b(because_b);
}

impl<I1: UserFacing, I2: UserFacing> UserFacing for FallingBack<I1, I2> {
    // In the <Limited, SoftwareImplementation> case, this will be needlessly big; we might
    // consider allowing SoftwareImplementation to do an uninhabited version for things it won't
    // need anyway.
    //
    // That's rather straightforward in here where A is expected to be ZST (at worst by having a
    // [Uninhabited; 0/1] field shown in `../../const-to-uninhabited/`). In real situations where
    // there could be some versions we need and some we don't need, things could become trickier --
    // but then, when all fails, we could still make A more fine-grained for specific composite
    // functions, or introduce AB types.
    type A = Either<I1::A, I2::A>;
    // In the <Limited, SoftwareImplementation> case, this is just SoftwareImplementation::B b/c
    // the other one is uninhabited.
    type B = Either<I1::B, I2::B>;

    // If there are too many of those, we might want to macro-generate them
    fn can_a(&mut self) -> Option<Self::A> {
        self.main.can_a().map(Either::Main).or_else(|| self.backup.can_a().map(Either::Backup))
    }
    fn can_b(&mut self) -> Option<Self::B> {
        self.main.can_b().map(Either::Main).or_else(|| self.backup.can_b().map(Either::Backup))
    }

    fn composite_function(&mut self, because_a: Self::A, because_b: Self::B) {
        // This is where the fun begins.
        //
        // Let's give either a chance to do whatever they optimized for:
        match (because_a, because_b) {
            // If the main can't do any of those at all, the branch uses uninhabited data and
            // vanishes.
            (Either::Main(because_a), Either::Main(because_b)) => self.main.composite_function(because_a, because_b),
            // This will only vanish if the optimization of our software implementation that allows
            // our A to be just I1::A by being deliberately uninhabited is chosen.
            (Either::Backup(because_a), Either::Backup(because_b)) => self.backup.composite_function(because_a, because_b),
            // At least one of those should vanish; the other is what we are doing this for.
            (Either::Main(because_a), Either::Backup(because_b)) => {
                run_fallback(&mut self.main, because_a, &mut self.backup, because_b)
            }
            (Either::Backup(because_a), Either::Main(because_b)) => {
                run_fallback(&mut self.backup, because_a, &mut self.main, because_b)
            }
        }
    }

    // Maybe macro those too
    fn piece_a(&mut self, because: Self::A) {
        match because {
            Either::Main(because) => self.main.piece_a(because),
            Either::Backup(because) => self.backup.piece_a(because),
        }
    }
    fn piece_b(&mut self, because: Self::B) {
        match because {
            Either::Main(because) => self.main.piece_b(because),
            Either::Backup(because) => self.backup.piece_b(because),
        }
    }
}
