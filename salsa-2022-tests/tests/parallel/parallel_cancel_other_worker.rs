//! Test that we can cancel a query if the current revision has been cancelled.

use crate::setup::Database;
use crate::setup::Knobs;
use salsa::ParallelDatabase;
use salsa_2022_tests::HasLogger;
use expect_test::expect;

pub(crate) trait Db: salsa::DbWithJar<Jar> + Knobs {}

impl<T: salsa::DbWithJar<Jar> + Knobs> Db for T {}

#[salsa::jar(db = Db)]
pub(crate) struct Jar(MyInput, a, b);

#[salsa::input(jar = Jar)]
pub(crate) struct MyInput {
    field: i32,
}

#[salsa::tracked(jar = Jar)]
pub(crate) fn a(db: &dyn Db, input: MyInput) -> i32 {
    std::thread::sleep(std::time::Duration::from_secs(3));
    input.field(db) * 20 + 2
}

#[salsa::tracked(jar = Jar)]
pub(crate) fn b(db: &dyn Db, input: MyInput) -> i32 {
    std::thread::sleep(std::time::Duration::from_secs(3));
    input.field(db) * 20 + 2
}


#[test]
fn execute() {
    let mut db = Database::default();
    db.knobs().signal_on_will_block.set(3);

    let input = MyInput::new(&db, 1);

    std::thread::spawn({
        let db = db.snapshot();
        move || a(&*db, input)
    });

    std::thread::spawn({
        let db = db.snapshot();
        move || b(&*db, input)
    });

    // change the input to cancel the query in other workers with old revision
    input.set_field(&mut db).to(2);

    db.assert_logs(expect![[r#"
        [
            "Event { runtime_id: RuntimeId { counter: 1 }, kind: DidCancellation }",
            "Event { runtime_id: RuntimeId { counter: 2 }, kind: DidCancellation }",
        ]"#]]);
}
