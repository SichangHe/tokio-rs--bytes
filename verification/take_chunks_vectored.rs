use vstd::prelude::*;

verus! {

pub open spec fn sum_lengths(lengths: Seq<usize>) -> nat
    decreases lengths.len(),
{
    if lengths.len() == 0 {
        0
    } else {
        sum_lengths(lengths.drop_last()) + lengths.last() as nat
    }
}

proof fn lemma_sum_push(lengths: Seq<usize>, length: usize)
    ensures
        sum_lengths(lengths.push(length)) == sum_lengths(lengths) + length as nat,
{
    assert(lengths.push(length).drop_last() == lengths);
}

proof fn lemma_take_step(lengths: Seq<usize>, count: nat)
    requires
        count < lengths.len(),
    ensures
        lengths.take(count as int).push(lengths[count as int])
            == lengths.take((count + 1) as int),
        sum_lengths(lengths.take((count + 1) as int))
            == sum_lengths(lengths.take(count as int)) + lengths[count as int] as nat,
{
    assert(lengths.take((count + 1) as int).drop_last() == lengths.take(count as int));
    assert(lengths.take((count + 1) as int).last() == lengths[count as int]);
}

proof fn lemma_sum_take_mono(lengths: Seq<usize>, from: nat, to: nat)
    requires
        from <= to <= lengths.len(),
    ensures
        sum_lengths(lengths.take(from as int)) <= sum_lengths(lengths.take(to as int)),
    decreases to - from,
{
    if from < to {
        lemma_sum_take_mono(lengths, from, (to - 1) as nat);
        lemma_take_step(lengths, (to - 1) as nat);
    }
}

pub open spec fn valid_take_result(
    source: Seq<usize>,
    dst_capacity: usize,
    limit: usize,
    result: Seq<usize>,
) -> bool {
    &&& result.len() <= source.len()
    &&& result.len() <= dst_capacity
    &&& result.len() <= 16
    &&& forall|i: int| 0 <= i < result.len() ==> result[i] <= source[i]
    &&& forall|i: int| 0 <= i && i + 1 < result.len() ==> result[i] == source[i]
    &&& sum_lengths(result) == if sum_lengths(source) <= limit {
        sum_lengths(source)
    } else {
        limit as nat
    }
    &&& sum_lengths(result) <= limit
    &&& (result.len() < source.len() ==> sum_lengths(result) == limit)
    &&& (limit == 0 || dst_capacity == 0 ==> result.len() == 0)
}

pub fn take_chunks_vectored_lengths(
    // @ASSUME:underlying_chunks_vectored
    source: &Vec<usize>,
    dst_capacity: usize,
    limit: usize,
) -> (result: Vec<usize>)
    requires
        source@.len() <= dst_capacity,
        source@.len() <= 16,
    ensures
        valid_take_result(source@, dst_capacity, limit, result@),
{
    let available = source.len();

    let mut result = Vec::new();
    if limit == 0 {
        return result;
    }

    let mut remaining = limit;
    let mut i = 0;
    while i < available
        invariant
            i <= available,
            available <= source.len(),
            available <= dst_capacity,
            available <= 16,
            source@.take(available as int) == source@,
            result@ == source@.take(i as int),
            sum_lengths(result@) + remaining as nat == limit as nat,
            remaining > 0,
        decreases available - i,
    {
        let length = source[i];
        if length >= remaining {
            proof {
                lemma_sum_push(result@, remaining);
            }
            result.push(remaining);
            proof {
                lemma_take_step(source@, i as nat);
                lemma_sum_take_mono(source@, (i + 1) as nat, available as nat);
            }
            assert(result@.len() <= source@.len());
            assert(result@.len() <= dst_capacity);
            assert(result@.len() <= 16);
            assert forall|j: int| 0 <= j < result@.len() implies result@[j] <= source@[j] by {
                if j < i {
                    assert(result@[j] == source@[j]);
                } else {
                    assert(j == i);
                }
            }
            assert forall|j: int| 0 <= j && j + 1 < result@.len() implies result@[j] == source@[j] by {
                assert(j < i);
            }
            assert(sum_lengths(result@) == limit as nat);
            assert(sum_lengths(source@.take(available as int)) >= limit as nat);
            assert(valid_take_result(source@, dst_capacity, limit, result@));
            return result;
        }

        proof {
            lemma_sum_push(result@, length);
        }
        result.push(length);
        proof {
            lemma_take_step(source@, i as nat);
        }
        remaining = remaining - length;
        i += 1;
    }
    assert(result@ == source@.take(available as int));
    assert(sum_lengths(result@) < limit as nat);
    assert forall|j: int| 0 <= j < result@.len() implies result@[j] <= source@[j] by {
        assert(result@[j] == source@[j]);
    }
    assert forall|j: int| 0 <= j && j + 1 < result@.len() implies result@[j] == source@[j] by {
    }
    assert(valid_take_result(source@, dst_capacity, limit, result@));
    result
}

fn main() {}

}
