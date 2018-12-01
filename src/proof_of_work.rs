use crate::block::Block;
use num::bigint::BigUint;
use num::traits::identities::One;

const TARGER_BIT: usize = 24;

struct ProofOfWork {
    block: Block,
    target: BigUint,
}

impl ProofOfWork {
    fn new(block: Block) -> Self {
        let target = BigUint::one() << (256 - TARGER_BIT);
        ProofOfWork { block, target }
    }
}
