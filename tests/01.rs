// We CAN conditionally compile here
//
// #[cfg(feature = "fastrand")]

use cami::prelude::Cami;
use cami_benches::data::{self, Data, OwnAndOut};
use cami_benches::outish::{OutCollectionVec, OutCollectionVecIndicator, OutIndicatorStrIndicator};
use cami_benches::testish::DataTest;

type OutType = &'static str;
type OutTypeRef = &'static [OutType];

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
