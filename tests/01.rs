// We CAN conditionally compile here
//
// #[cfg(feature = "fastrand")]

use cami::prelude::Cami;
use cami_benches::data::{self, Data, OwnAndOut};
use cami_benches::outish::{OutCollectionVec, OutCollectionVecIndicator, OutIndicatorStrIndicator};
use cami_benches::testish::DataTest;

type OutType = &'static str;
type OutTypeRef = &'static [OutType];

fn out_from<InType, OwnType, OutType>(
    vec: Vec<InType>,
    generate_own_item: impl Fn(&mut DataTest<InType>) -> OwnType,
    generate_out_item: impl Fn(&'static OwnType) -> OutType,
    allows_multiple_equal_items: bool,
) -> &'static [OwnType] {
    let mut data = DataTest::<InType>::new(vec);

    let own_and_out =
        OwnAndOut::new_for_data(&mut data, |d| d.string(), |string| &string[..], true);
    //data::purge_cache();
    own_and_out.out
}

fn out() -> OutTypeRef {
    let mut data = DataTest::<String>::new(
        vec!["ckp"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
    );

    let own_and_out = OwnAndOut::new_for_data(
        d,
        |d: &mut DataTest<String>| data.string(),
        |string| &string[..],
        true,
    );
    //data::purge_cache();
    own_and_out.out
}
