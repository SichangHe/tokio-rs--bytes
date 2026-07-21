use vstd::prelude::*;

verus! {

pub struct ChainCursor {
    pub a_remaining: usize,
    pub b_remaining: usize,
}

impl ChainCursor {
    pub open spec fn total(self) -> nat {
        self.a_remaining as nat + self.b_remaining as nat
    }

    pub fn remaining(&self) -> (remaining: usize)
        ensures
            remaining as nat == if self.total() <= usize::MAX as nat {
                self.total()
            } else {
                usize::MAX as nat
            },
    {
        self.a_remaining.saturating_add(self.b_remaining)
    }

    pub fn advance(&mut self, mut cnt: usize)
        requires
            cnt as nat <= old(self).total(),
        ensures
            self.total() == old(self).total() - cnt as nat,
            cnt <= old(self).a_remaining ==> self.a_remaining as nat + cnt as nat
                == old(self).a_remaining as nat,
            cnt <= old(self).a_remaining ==> self.b_remaining == old(self).b_remaining,
            cnt > old(self).a_remaining ==> self.a_remaining == 0,
            cnt > old(self).a_remaining ==> self.b_remaining as nat + cnt as nat
                == old(self).a_remaining as nat + old(self).b_remaining as nat,
    {
        // @ASSUME:underlying_buf_advance
        let a_rem = self.a_remaining;

        if a_rem != 0 {
            if a_rem >= cnt {
                self.a_remaining = self.a_remaining - cnt;
                return;
            }

            self.a_remaining = 0;
            cnt = cnt - a_rem;
        }

        self.b_remaining = self.b_remaining - cnt;
    }
}

fn main() {}

}
