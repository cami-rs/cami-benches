use crate::outish::{
    Out, OutCollRetriever, OutCollRetrieverCami, OutCollection, OutCollectionIndicator,
    OutIndicatorIndicator, OutRetriever,
};
use cami::Cami;

/// Collect and sort.
pub fn lexi_stable<
    'out,
    SubType: Out + 'out,
    OutIndicatorIndicatorImpl: OutIndicatorIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
>(
    out: &'out [OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>],
) -> OutCollRetriever<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType> {
    lexi_indicated::<
        OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>,
        OutCollRetriever<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType>,
    >(out, true)
}
/// Collect and sort unstable. If the collection doesn't support unstable sort, this may [panic].
pub fn lexi_unstable<
    'out,
    SubType: Out + 'out,
    OutIndicatorIndicatorImpl: OutIndicatorIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
>(
    out: &'out [OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>],
) -> OutCollRetriever<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType> {
    lexi_indicated::<
        OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>,
        OutCollRetriever<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType>,
    >(out, false)
}
/// Collect. If the collection doesn't keep sorted order, then this does NOT sort.
pub fn lexi_indicated<
    'out,
    OutType: Out + 'out,
    OutCollectionLexi: OutCollection<'out, OutType>,
>(
    out: &'out [OutType],
    stable_sort: bool,
) -> OutCollectionLexi {
    let mut col = OutCollectionLexi::with_capacity(out.len());
    col.extend(out.iter().cloned());
    if stable_sort {
        col.sort();
    } else {
        col.sort_unstable();
    }
    col
}

/// Collect [Cami] wrapers around items and sort.
pub fn cami_stable<
    'out,
    SubType: Out + 'out,
    OutIndicatorIndicatorImpl: OutIndicatorIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
>(
    out: &'out [OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>],
) -> OutCollRetrieverCami<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType> {
    cami_indicated::<
        OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>,
        OutCollRetrieverCami<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType>,
    >(out, true)
}
/// Collect and sort unstable. If the collection doesn't support unstable sort, this may [panic].
pub fn cami_unstable<
    'out,
    SubType: Out + 'out,
    OutIndicatorIndicatorImpl: OutIndicatorIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
>(
    out: &'out [OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>],
) -> OutCollRetrieverCami<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType> {
    cami_indicated::<
        OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>,
        OutCollRetrieverCami<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType>,
    >(out, false)
}
/// Collect. If the collection doesn't keep sorted order, then this does NOT sort.
pub fn cami_indicated<
    'out,
    OutType: Out + 'out,
    OutCollectionCami: OutCollection<'out, Cami<OutType>>,
>(
    out: &'out [OutType],
    stable_sort: bool,
) -> OutCollectionCami {
    let mut col = OutCollectionCami::with_capacity(out.len());
    col.extend(out.iter().cloned().map(Cami::new));
    if stable_sort {
        col.sort();
    } else {
        col.sort_unstable();
    }
    col
}
