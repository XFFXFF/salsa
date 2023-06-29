#![feature(prelude_import)]
//! Test that a `tracked` fn on a `salsa::input`
//! compiles and executes successfully.
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use salsa_2022_tests::{HasLogger, Logger};
use expect_test::expect;
use test_log::test;
struct Jar(
    <MyInput as salsa::storage::IngredientsFor>::Ingredients,
    <MyTracked as salsa::storage::IngredientsFor>::Ingredients,
    <final_result as salsa::storage::IngredientsFor>::Ingredients,
    <intermediate_result as salsa::storage::IngredientsFor>::Ingredients,
);
impl salsa::storage::HasIngredientsFor<MyInput> for Jar {
    fn ingredient(&self) -> &<MyInput as salsa::storage::IngredientsFor>::Ingredients {
        &self.0
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <MyInput as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.0
    }
}
impl salsa::storage::HasIngredientsFor<MyTracked> for Jar {
    fn ingredient(&self) -> &<MyTracked as salsa::storage::IngredientsFor>::Ingredients {
        &self.1
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <MyTracked as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.1
    }
}
impl salsa::storage::HasIngredientsFor<final_result> for Jar {
    fn ingredient(
        &self,
    ) -> &<final_result as salsa::storage::IngredientsFor>::Ingredients {
        &self.2
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <final_result as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.2
    }
}
impl salsa::storage::HasIngredientsFor<intermediate_result> for Jar {
    fn ingredient(
        &self,
    ) -> &<intermediate_result as salsa::storage::IngredientsFor>::Ingredients {
        &self.3
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <intermediate_result as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.3
    }
}
impl<'salsa_db> salsa::jar::Jar<'salsa_db> for Jar {
    type DynDb = dyn Db + 'salsa_db;
    fn create_jar<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self
    where
        DB: salsa::storage::JarFromJars<Self> + salsa::storage::DbWithJar<Self>,
    {
        let i0 = <MyInput as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i1 = <MyTracked as salsa::storage::IngredientsFor>::create_ingredients(
            routes,
        );
        let i2 = <final_result as salsa::storage::IngredientsFor>::create_ingredients(
            routes,
        );
        let i3 = <intermediate_result as salsa::storage::IngredientsFor>::create_ingredients(
            routes,
        );
        Self(i0, i1, i2, i3)
    }
}
trait Db: salsa::DbWithJar<Jar> + HasLogger {}
struct MyInput(salsa::Id);
#[automatically_derived]
impl ::core::marker::Copy for MyInput {}
#[automatically_derived]
impl ::core::clone::Clone for MyInput {
    #[inline]
    fn clone(&self) -> MyInput {
        let _: ::core::clone::AssertParamIsClone<salsa::Id>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for MyInput {}
#[automatically_derived]
impl ::core::cmp::PartialEq for MyInput {
    #[inline]
    fn eq(&self, other: &MyInput) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for MyInput {
    #[inline]
    fn partial_cmp(
        &self,
        other: &MyInput,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for MyInput {}
#[automatically_derived]
impl ::core::cmp::Eq for MyInput {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<salsa::Id>;
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for MyInput {
    #[inline]
    fn cmp(&self, other: &MyInput) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}
#[automatically_derived]
impl ::core::hash::Hash for MyInput {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for MyInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "MyInput", &&self.0)
    }
}
impl MyInput {
    pub fn new(__db: &<Jar as salsa::jar::Jar<'_>>::DynDb, field: u32) -> Self {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar(__db);
        let __ingredients = <Jar as salsa::storage::HasIngredientsFor<
            MyInput,
        >>::ingredient(__jar);
        let __id = __ingredients.1.new_input(__runtime);
        __ingredients.0.store_new(__runtime, __id, field, salsa::Durability::LOW);
        __id
    }
    fn field<'db>(self, __db: &'db <Jar as salsa::jar::Jar<'_>>::DynDb) -> u32 {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar(__db);
        let __ingredients = <Jar as salsa::storage::HasIngredientsFor<
            MyInput,
        >>::ingredient(__jar);
        __ingredients.0.fetch(__runtime, self).clone()
    }
    fn set_field<'db>(
        self,
        __db: &'db mut <Jar as salsa::jar::Jar<'_>>::DynDb,
    ) -> salsa::setter::Setter<'db, MyInput, u32> {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar_mut(__db);
        let __ingredients = <Jar as salsa::storage::HasIngredientsFor<
            MyInput,
        >>::ingredient_mut(__jar);
        salsa::setter::Setter::new(__runtime, self, &mut __ingredients.0)
    }
}
impl salsa::storage::IngredientsFor for MyInput {
    type Jar = Jar;
    type Ingredients = (
        salsa::input_field::InputFieldIngredient<MyInput, u32>,
        salsa::input::InputIngredient<MyInput>,
    );
    fn create_ingredients<DB>(
        routes: &mut salsa::routes::Routes<DB>,
    ) -> Self::Ingredients
    where
        DB: salsa::DbWithJar<Self::Jar> + salsa::storage::JarFromJars<Self::Jar>,
    {
        (
            {
                let index = routes
                    .push(
                        |jars| {
                            let jar = <DB as salsa::storage::JarFromJars<
                                Self::Jar,
                            >>::jar_from_jars(jars);
                            let ingredients = <_ as salsa::storage::HasIngredientsFor<
                                Self,
                            >>::ingredient(jar);
                            &ingredients.0
                        },
                        |jars| {
                            let jar = <DB as salsa::storage::JarFromJars<
                                Self::Jar,
                            >>::jar_from_jars_mut(jars);
                            let ingredients = <_ as salsa::storage::HasIngredientsFor<
                                Self,
                            >>::ingredient_mut(jar);
                            &mut ingredients.0
                        },
                    );
                salsa::input_field::InputFieldIngredient::new(index, "field")
            },
            {
                let index = routes
                    .push(
                        |jars| {
                            let jar = <DB as salsa::storage::JarFromJars<
                                Self::Jar,
                            >>::jar_from_jars(jars);
                            let ingredients = <_ as salsa::storage::HasIngredientsFor<
                                Self,
                            >>::ingredient(jar);
                            &ingredients.1
                        },
                        |jars| {
                            let jar = <DB as salsa::storage::JarFromJars<
                                Self::Jar,
                            >>::jar_from_jars_mut(jars);
                            let ingredients = <_ as salsa::storage::HasIngredientsFor<
                                Self,
                            >>::ingredient_mut(jar);
                            &mut ingredients.1
                        },
                    );
                salsa::input::InputIngredient::new(index, "MyInput")
            },
        )
    }
}
impl salsa::AsId for MyInput {
    fn as_id(self) -> salsa::Id {
        self.0
    }
    fn from_id(id: salsa::Id) -> Self {
        MyInput(id)
    }
}
impl ::salsa::DebugWithDb<<Jar as salsa::jar::Jar<'_>>::DynDb> for MyInput {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        _db: &<Jar as salsa::jar::Jar<'_>>::DynDb,
        _include_all_fields: bool,
    ) -> ::std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        let mut debug_struct = &mut f.debug_struct("MyInput");
        debug_struct = debug_struct.field("[salsa id]", &self.0.as_u32());
        if _include_all_fields {
            debug_struct = debug_struct
                .field(
                    "field",
                    &::salsa::debug::helper::SalsaDebug::<
                        u32,
                        <Jar as salsa::jar::Jar<'_>>::DynDb,
                    >::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        &self.field(_db),
                        _db,
                        _include_all_fields,
                    ),
                );
        }
        debug_struct.finish()
    }
}
impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for MyInput
where
    DB: ?Sized + salsa::DbWithJar<Jar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}
#[allow(non_camel_case_types)]
struct final_result {
    intern_map: salsa::interned::IdentityInterner<MyInput>,
    function: salsa::function::FunctionIngredient<Self>,
}
impl salsa::function::Configuration for final_result {
    type Jar = Jar;
    type SalsaStruct = MyInput;
    type Key = MyInput;
    type Value = u32;
    const CYCLE_STRATEGY: salsa::cycle::CycleRecoveryStrategy = salsa::cycle::CycleRecoveryStrategy::Panic;
    fn should_backdate_value(v1: &Self::Value, v2: &Self::Value) -> bool {
        salsa::function::should_backdate_value(v1, v2)
    }
    fn execute(__db: &salsa::function::DynDb<Self>, __id: Self::Key) -> Self::Value {
        fn __fn(db: &dyn Db, input: MyInput) -> u32 {
            db.push_log({
                let res = ::alloc::fmt::format(
                    format_args!("final_result({0:?})", input),
                );
                res
            });
            intermediate_result(db, input).field(db) * 2
        }
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar(__db);
        let __ingredients = <_ as salsa::storage::HasIngredientsFor<
            final_result,
        >>::ingredient(__jar);
        let __key = __ingredients.intern_map.data(__runtime, __id).clone();
        __fn(__db, __key.0)
    }
    fn recover_from_cycle(
        _db: &salsa::function::DynDb<Self>,
        _cycle: &salsa::Cycle,
        _key: Self::Key,
    ) -> Self::Value {
        ::core::panicking::panic("explicit panic")
    }
}
impl salsa::storage::IngredientsFor for final_result {
    type Ingredients = Self;
    type Jar = Jar;
    fn create_ingredients<DB>(
        routes: &mut salsa::routes::Routes<DB>,
    ) -> Self::Ingredients
    where
        DB: salsa::DbWithJar<Self::Jar> + salsa::storage::JarFromJars<Self::Jar>,
    {
        Self {
            intern_map: salsa::interned::IdentityInterner::new(),
            function: {
                let index = routes
                    .push(
                        |jars| {
                            let jar = <DB as salsa::storage::JarFromJars<
                                Self::Jar,
                            >>::jar_from_jars(jars);
                            let ingredients = <_ as salsa::storage::HasIngredientsFor<
                                Self::Ingredients,
                            >>::ingredient(jar);
                            &ingredients.function
                        },
                        |jars| {
                            let jar = <DB as salsa::storage::JarFromJars<
                                Self::Jar,
                            >>::jar_from_jars_mut(jars);
                            let ingredients = <_ as salsa::storage::HasIngredientsFor<
                                Self::Ingredients,
                            >>::ingredient_mut(jar);
                            &mut ingredients.function
                        },
                    );
                let ingredient = salsa::function::FunctionIngredient::new(
                    index,
                    "final_result",
                );
                ingredient.set_capacity(0usize);
                ingredient
            },
        }
    }
}
impl final_result {
    #[allow(dead_code, clippy::needless_lifetimes)]
    fn get<'__db>(db: &'__db dyn Db, input: MyInput) -> &'__db u32 {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar(db);
        let __ingredients = <_ as salsa::storage::HasIngredientsFor<
            final_result,
        >>::ingredient(__jar);
        let __key = __ingredients.intern_map.intern(__runtime, (input));
        __ingredients.function.fetch(db, __key)
    }
    #[allow(dead_code, clippy::needless_lifetimes)]
    fn set(db: &mut dyn Db, input: MyInput, __value: u32) {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar_mut(db);
        let __ingredients = <_ as salsa::storage::HasIngredientsFor<
            final_result,
        >>::ingredient_mut(__jar);
        let __key = __ingredients.intern_map.intern(__runtime, (input));
        __ingredients.function.store(__runtime, __key, __value, salsa::Durability::LOW)
    }
    #[allow(dead_code, clippy::needless_lifetimes)]
    fn accumulated<'__db, __A: salsa::accumulator::Accumulator>(
        db: &'__db dyn Db,
        input: MyInput,
    ) -> Vec<<__A as salsa::accumulator::Accumulator>::Data>
    where
        <Jar as salsa::jar::Jar<
            '__db,
        >>::DynDb: salsa::storage::HasJar<<__A as salsa::accumulator::Accumulator>::Jar>,
    {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar(db);
        let __ingredients = <_ as salsa::storage::HasIngredientsFor<
            final_result,
        >>::ingredient(__jar);
        let __key = __ingredients.intern_map.intern(__runtime, (input));
        __ingredients.function.accumulated::<__A>(db, __key)
    }
}
#[allow(clippy::needless_lifetimes)]
fn final_result(db: &dyn Db, input: MyInput) -> u32 {
    Clone::clone(final_result::get(db, input))
}
struct __MyTrackedField(std::convert::Infallible);
#[automatically_derived]
impl ::core::marker::Copy for __MyTrackedField {}
#[automatically_derived]
impl ::core::clone::Clone for __MyTrackedField {
    #[inline]
    fn clone(&self) -> __MyTrackedField {
        let _: ::core::clone::AssertParamIsClone<std::convert::Infallible>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for __MyTrackedField {}
#[automatically_derived]
impl ::core::cmp::PartialEq for __MyTrackedField {
    #[inline]
    fn eq(&self, other: &__MyTrackedField) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for __MyTrackedField {
    #[inline]
    fn partial_cmp(
        &self,
        other: &__MyTrackedField,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for __MyTrackedField {}
#[automatically_derived]
impl ::core::cmp::Eq for __MyTrackedField {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<std::convert::Infallible>;
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for __MyTrackedField {
    #[inline]
    fn cmp(&self, other: &__MyTrackedField) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}
#[automatically_derived]
impl ::core::hash::Hash for __MyTrackedField {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for __MyTrackedField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(
            f,
            "__MyTrackedField",
            &&self.0,
        )
    }
}
struct MyTracked(salsa::Id);
#[automatically_derived]
impl ::core::marker::Copy for MyTracked {}
#[automatically_derived]
impl ::core::clone::Clone for MyTracked {
    #[inline]
    fn clone(&self) -> MyTracked {
        let _: ::core::clone::AssertParamIsClone<salsa::Id>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for MyTracked {}
#[automatically_derived]
impl ::core::cmp::PartialEq for MyTracked {
    #[inline]
    fn eq(&self, other: &MyTracked) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for MyTracked {
    #[inline]
    fn partial_cmp(
        &self,
        other: &MyTracked,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for MyTracked {}
#[automatically_derived]
impl ::core::cmp::Eq for MyTracked {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<salsa::Id>;
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for MyTracked {
    #[inline]
    fn cmp(&self, other: &MyTracked) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}
#[automatically_derived]
impl ::core::hash::Hash for MyTracked {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for MyTracked {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "MyTracked", &&self.0)
    }
}
impl MyTracked {
    pub fn new(__db: &<Jar as salsa::jar::Jar<'_>>::DynDb, field: u32) -> Self {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar(__db);
        let __ingredients = <Jar as salsa::storage::HasIngredientsFor<
            MyTracked,
        >>::ingredient(__jar);
        let __id = __ingredients.1.new_struct(__runtime, ());
        __ingredients.0.specify_and_record(__db, __id, field);
        __id
    }
    fn field<'db>(self, __db: &'db <Jar as salsa::jar::Jar<'_>>::DynDb) -> u32 {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar(__db);
        let __ingredients = <Jar as salsa::storage::HasIngredientsFor<
            MyTracked,
        >>::ingredient(__jar);
        __ingredients.0.fetch(__db, self).clone()
    }
}
impl salsa::storage::IngredientsFor for MyTracked {
    type Jar = Jar;
    type Ingredients = (
        salsa::function::FunctionIngredient<__MyTrackedField>,
        salsa::tracked_struct::TrackedStructIngredient<MyTracked, ()>,
    );
    fn create_ingredients<DB>(
        routes: &mut salsa::routes::Routes<DB>,
    ) -> Self::Ingredients
    where
        DB: salsa::DbWithJar<Self::Jar> + salsa::storage::JarFromJars<Self::Jar>,
    {
        (
            {
                let index = routes
                    .push(
                        |jars| {
                            let jar = <DB as salsa::storage::JarFromJars<
                                Self::Jar,
                            >>::jar_from_jars(jars);
                            let ingredients = <_ as salsa::storage::HasIngredientsFor<
                                Self,
                            >>::ingredient(jar);
                            &ingredients.0
                        },
                        |jars| {
                            let jar = <DB as salsa::storage::JarFromJars<
                                Self::Jar,
                            >>::jar_from_jars_mut(jars);
                            let ingredients = <_ as salsa::storage::HasIngredientsFor<
                                Self,
                            >>::ingredient_mut(jar);
                            &mut ingredients.0
                        },
                    );
                salsa::function::FunctionIngredient::new(index, "field")
            },
            {
                let index = routes
                    .push(
                        |jars| {
                            let jar = <DB as salsa::storage::JarFromJars<
                                Self::Jar,
                            >>::jar_from_jars(jars);
                            let ingredients = <_ as salsa::storage::HasIngredientsFor<
                                Self,
                            >>::ingredient(jar);
                            &ingredients.1
                        },
                        |jars| {
                            let jar = <DB as salsa::storage::JarFromJars<
                                Self::Jar,
                            >>::jar_from_jars_mut(jars);
                            let ingredients = <_ as salsa::storage::HasIngredientsFor<
                                Self,
                            >>::ingredient_mut(jar);
                            &mut ingredients.1
                        },
                    );
                salsa::tracked_struct::TrackedStructIngredient::new(index, "MyTracked")
            },
        )
    }
}
impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for MyTracked
where
    DB: ?Sized + salsa::DbWithJar<Jar>,
{
    fn register_dependent_fn(db: &DB, index: salsa::routes::IngredientIndex) {
        let (jar, _) = <_ as salsa::storage::HasJar<Jar>>::jar(db);
        let ingredients = <Jar as salsa::storage::HasIngredientsFor<
            MyTracked,
        >>::ingredient(jar);
        ingredients.1.register_dependent_fn(index)
    }
}
impl<DB> salsa::tracked_struct::TrackedStructInDb<DB> for MyTracked
where
    DB: ?Sized + salsa::DbWithJar<Jar>,
{
    fn database_key_index(self, db: &DB) -> salsa::DatabaseKeyIndex {
        let (jar, _) = <_ as salsa::storage::HasJar<Jar>>::jar(db);
        let ingredients = <Jar as salsa::storage::HasIngredientsFor<
            MyTracked,
        >>::ingredient(jar);
        ingredients.1.database_key_index(self)
    }
}
impl salsa::AsId for MyTracked {
    fn as_id(self) -> salsa::Id {
        self.0
    }
    fn from_id(id: salsa::Id) -> Self {
        MyTracked(id)
    }
}
impl ::salsa::DebugWithDb<<Jar as salsa::jar::Jar<'_>>::DynDb> for MyTracked {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        _db: &<Jar as salsa::jar::Jar<'_>>::DynDb,
        _include_all_fields: bool,
    ) -> ::std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        let mut debug_struct = &mut f.debug_struct("MyTracked");
        debug_struct = debug_struct.field("[salsa id]", &self.0.as_u32());
        if _include_all_fields {
            debug_struct = debug_struct
                .field(
                    "field",
                    &::salsa::debug::helper::SalsaDebug::<
                        u32,
                        <Jar as salsa::jar::Jar<'_>>::DynDb,
                    >::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        &self.field(_db),
                        _db,
                        _include_all_fields,
                    ),
                );
        }
        debug_struct.finish()
    }
}
impl salsa::function::Configuration for __MyTrackedField {
    type Jar = Jar;
    type SalsaStruct = MyTracked;
    type Key = MyTracked;
    type Value = u32;
    const CYCLE_STRATEGY: salsa::cycle::CycleRecoveryStrategy = salsa::cycle::CycleRecoveryStrategy::Panic;
    fn should_backdate_value(v1: &Self::Value, v2: &Self::Value) -> bool {
        salsa::function::should_backdate_value(v1, v2)
    }
    fn execute(db: &salsa::function::DynDb<Self>, key: Self::Key) -> Self::Value {
        {
            ::core::panicking::panic_fmt(
                format_args!("`execute` method for field `MyTracked::field` invoked"),
            );
        }
    }
    fn recover_from_cycle(
        db: &salsa::function::DynDb<Self>,
        cycle: &salsa::Cycle,
        key: Self::Key,
    ) -> Self::Value {
        {
            ::core::panicking::panic_fmt(
                format_args!("`execute` method for field `MyTracked::field` invoked"),
            );
        }
    }
}
#[allow(non_camel_case_types)]
struct intermediate_result {
    intern_map: salsa::interned::IdentityInterner<MyInput>,
    function: salsa::function::FunctionIngredient<Self>,
}
impl salsa::function::Configuration for intermediate_result {
    type Jar = Jar;
    type SalsaStruct = MyInput;
    type Key = MyInput;
    type Value = MyTracked;
    const CYCLE_STRATEGY: salsa::cycle::CycleRecoveryStrategy = salsa::cycle::CycleRecoveryStrategy::Panic;
    fn should_backdate_value(v1: &Self::Value, v2: &Self::Value) -> bool {
        salsa::function::should_backdate_value(v1, v2)
    }
    fn execute(__db: &salsa::function::DynDb<Self>, __id: Self::Key) -> Self::Value {
        fn __fn(db: &dyn Db, input: MyInput) -> MyTracked {
            db.push_log({
                let res = ::alloc::fmt::format(
                    format_args!("intermediate_result({0:?})", input),
                );
                res
            });
            MyTracked::new(db, input.field(db) / 2)
        }
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar(__db);
        let __ingredients = <_ as salsa::storage::HasIngredientsFor<
            intermediate_result,
        >>::ingredient(__jar);
        let __key = __ingredients.intern_map.data(__runtime, __id).clone();
        __fn(__db, __key.0)
    }
    fn recover_from_cycle(
        _db: &salsa::function::DynDb<Self>,
        _cycle: &salsa::Cycle,
        _key: Self::Key,
    ) -> Self::Value {
        ::core::panicking::panic("explicit panic")
    }
}
impl salsa::storage::IngredientsFor for intermediate_result {
    type Ingredients = Self;
    type Jar = Jar;
    fn create_ingredients<DB>(
        routes: &mut salsa::routes::Routes<DB>,
    ) -> Self::Ingredients
    where
        DB: salsa::DbWithJar<Self::Jar> + salsa::storage::JarFromJars<Self::Jar>,
    {
        Self {
            intern_map: salsa::interned::IdentityInterner::new(),
            function: {
                let index = routes
                    .push(
                        |jars| {
                            let jar = <DB as salsa::storage::JarFromJars<
                                Self::Jar,
                            >>::jar_from_jars(jars);
                            let ingredients = <_ as salsa::storage::HasIngredientsFor<
                                Self::Ingredients,
                            >>::ingredient(jar);
                            &ingredients.function
                        },
                        |jars| {
                            let jar = <DB as salsa::storage::JarFromJars<
                                Self::Jar,
                            >>::jar_from_jars_mut(jars);
                            let ingredients = <_ as salsa::storage::HasIngredientsFor<
                                Self::Ingredients,
                            >>::ingredient_mut(jar);
                            &mut ingredients.function
                        },
                    );
                let ingredient = salsa::function::FunctionIngredient::new(
                    index,
                    "intermediate_result",
                );
                ingredient.set_capacity(0usize);
                ingredient
            },
        }
    }
}
impl intermediate_result {
    #[allow(dead_code, clippy::needless_lifetimes)]
    fn get<'__db>(db: &'__db dyn Db, input: MyInput) -> &'__db MyTracked {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar(db);
        let __ingredients = <_ as salsa::storage::HasIngredientsFor<
            intermediate_result,
        >>::ingredient(__jar);
        let __key = __ingredients.intern_map.intern(__runtime, (input));
        __ingredients.function.fetch(db, __key)
    }
    #[allow(dead_code, clippy::needless_lifetimes)]
    fn set(db: &mut dyn Db, input: MyInput, __value: MyTracked) {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar_mut(db);
        let __ingredients = <_ as salsa::storage::HasIngredientsFor<
            intermediate_result,
        >>::ingredient_mut(__jar);
        let __key = __ingredients.intern_map.intern(__runtime, (input));
        __ingredients.function.store(__runtime, __key, __value, salsa::Durability::LOW)
    }
    #[allow(dead_code, clippy::needless_lifetimes)]
    fn accumulated<'__db, __A: salsa::accumulator::Accumulator>(
        db: &'__db dyn Db,
        input: MyInput,
    ) -> Vec<<__A as salsa::accumulator::Accumulator>::Data>
    where
        <Jar as salsa::jar::Jar<
            '__db,
        >>::DynDb: salsa::storage::HasJar<<__A as salsa::accumulator::Accumulator>::Jar>,
    {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar(db);
        let __ingredients = <_ as salsa::storage::HasIngredientsFor<
            intermediate_result,
        >>::ingredient(__jar);
        let __key = __ingredients.intern_map.intern(__runtime, (input));
        __ingredients.function.accumulated::<__A>(db, __key)
    }
}
#[allow(clippy::needless_lifetimes)]
fn intermediate_result(db: &dyn Db, input: MyInput) -> MyTracked {
    Clone::clone(intermediate_result::get(db, input))
}
struct Database {
    storage: salsa::Storage<Self>,
    logger: Logger,
}
#[automatically_derived]
impl ::core::default::Default for Database {
    #[inline]
    fn default() -> Database {
        Database {
            storage: ::core::default::Default::default(),
            logger: ::core::default::Default::default(),
        }
    }
}
impl salsa::database::AsSalsaDatabase for Database {
    fn as_salsa_database(&self) -> &dyn salsa::Database {
        self
    }
}
impl salsa::storage::HasJars for Database {
    type Jars = (Jar,);
    fn jars(&self) -> (&Self::Jars, &salsa::Runtime) {
        self.storage.jars()
    }
    fn jars_mut(&mut self) -> (&mut Self::Jars, &mut salsa::Runtime) {
        self.storage.jars_mut()
    }
    fn create_jars(routes: &mut salsa::routes::Routes<Self>) -> Self::Jars {
        (<Jar as salsa::jar::Jar>::create_jar(routes),)
    }
}
impl salsa::storage::HasJarsDyn for Database {
    fn runtime(&self) -> &salsa::Runtime {
        self.storage.runtime()
    }
    fn runtime_mut(&mut self) -> &mut salsa::Runtime {
        self.storage.runtime_mut()
    }
    fn maybe_changed_after(
        &self,
        input: salsa::key::DependencyIndex,
        revision: salsa::Revision,
    ) -> bool {
        let ingredient = self.storage.ingredient(input.ingredient_index());
        ingredient.maybe_changed_after(self, input, revision)
    }
    fn cycle_recovery_strategy(
        &self,
        ingredient_index: salsa::IngredientIndex,
    ) -> salsa::cycle::CycleRecoveryStrategy {
        let ingredient = self.storage.ingredient(ingredient_index);
        ingredient.cycle_recovery_strategy()
    }
    fn origin(
        &self,
        index: salsa::DatabaseKeyIndex,
    ) -> Option<salsa::runtime::local_state::QueryOrigin> {
        let ingredient = self.storage.ingredient(index.ingredient_index());
        ingredient.origin(index.key_index())
    }
    fn mark_validated_output(
        &self,
        executor: salsa::DatabaseKeyIndex,
        output: salsa::key::DependencyIndex,
    ) {
        let ingredient = self.storage.ingredient(output.ingredient_index());
        ingredient.mark_validated_output(self, executor, output.key_index());
    }
    fn remove_stale_output(
        &self,
        executor: salsa::DatabaseKeyIndex,
        stale_output: salsa::key::DependencyIndex,
    ) {
        let ingredient = self.storage.ingredient(stale_output.ingredient_index());
        ingredient.remove_stale_output(self, executor, stale_output.key_index());
    }
    fn salsa_struct_deleted(&self, ingredient: salsa::IngredientIndex, id: salsa::Id) {
        let ingredient = self.storage.ingredient(ingredient);
        ingredient.salsa_struct_deleted(self, id);
    }
    fn fmt_index(
        &self,
        index: salsa::key::DependencyIndex,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let ingredient = self.storage.ingredient(index.ingredient_index());
        ingredient.fmt_index(index.key_index(), self, fmt)
    }
}
impl salsa::storage::DbWithJar<Jar> for Database {
    fn as_jar_db<'db>(&'db self) -> &'db <Jar as salsa::jar::Jar<'db>>::DynDb
    where
        'db: 'db,
    {
        self as &'db <Jar as salsa::jar::Jar<'db>>::DynDb
    }
}
impl salsa::storage::HasJar<Jar> for Database {
    fn jar(&self) -> (&Jar, &salsa::Runtime) {
        let (__jars, __runtime) = self.storage.jars();
        (&__jars.0, __runtime)
    }
    fn jar_mut(&mut self) -> (&mut Jar, &mut salsa::Runtime) {
        let (__jars, __runtime) = self.storage.jars_mut();
        (&mut __jars.0, __runtime)
    }
}
impl salsa::storage::JarFromJars<Jar> for Database {
    fn jar_from_jars<'db>(jars: &Self::Jars) -> &Jar {
        &jars.0
    }
    fn jar_from_jars_mut<'db>(jars: &mut Self::Jars) -> &mut Jar {
        &mut jars.0
    }
}
impl salsa::Database for Database {}
impl Db for Database {}
impl HasLogger for Database {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "execute"]
pub const execute: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("execute"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "salsa-2022-tests/tests/hello_world.rs",
        start_line: 54usize,
        start_col: 4usize,
        end_line: 54usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(execute())),
};
fn execute() {
    fn test_impl() {
        {
            let mut db = Database::default();
            let input = MyInput::new(&db, 22);
            match (&final_result(&db, input), &22) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            db.assert_logs(::expect_test::Expect {
                position: ::expect_test::Position {
                    file: "salsa-2022-tests/tests/hello_world.rs",
                    line: 59u32,
                    column: 20u32,
                },
                data: r#"
        [
            "final_result(MyInput(Id { value: 1 }))",
            "intermediate_result(MyInput(Id { value: 1 }))",
        ]"#,
                indent: true,
            });
            input.set_field(&mut db).to(23);
            match (&final_result(&db, input), &22) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            db.assert_logs(::expect_test::Expect {
                position: ::expect_test::Position {
                    file: "salsa-2022-tests/tests/hello_world.rs",
                    line: 69u32,
                    column: 20u32,
                },
                data: r#"
        [
            "intermediate_result(MyInput(Id { value: 1 }))",
        ]"#,
                indent: true,
            });
            input.set_field(&mut db).to(24);
            match (&final_result(&db, input), &24) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            db.assert_logs(::expect_test::Expect {
                position: ::expect_test::Position {
                    file: "salsa-2022-tests/tests/hello_world.rs",
                    line: 76u32,
                    column: 20u32,
                },
                data: r#"
        [
            "intermediate_result(MyInput(Id { value: 1 }))",
            "final_result(MyInput(Id { value: 1 }))",
        ]"#,
                indent: true,
            });
        }
    }
    mod init {
        pub fn init() {
            {
                let _ = ::env_logger::builder().is_test(true).try_init();
            }
        }
    }
    init::init();
    test_impl()
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "red_herring"]
pub const red_herring: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("red_herring"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "salsa-2022-tests/tests/hello_world.rs",
        start_line: 85usize,
        start_col: 4usize,
        end_line: 85usize,
        end_col: 15usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(red_herring())),
};
/// Create and mutate a distinct input. No re-execution required.
fn red_herring() {
    fn test_impl() {
        {
            let mut db = Database::default();
            let input = MyInput::new(&db, 22);
            match (&final_result(&db, input), &22) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            db.assert_logs(::expect_test::Expect {
                position: ::expect_test::Position {
                    file: "salsa-2022-tests/tests/hello_world.rs",
                    line: 90u32,
                    column: 20u32,
                },
                data: r#"
        [
            "final_result(MyInput(Id { value: 1 }))",
            "intermediate_result(MyInput(Id { value: 1 }))",
        ]"#,
                indent: true,
            });
            let input2 = MyInput::new(&db, 44);
            input2.set_field(&mut db).to(66);
            match (&final_result(&db, input), &22) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            db.assert_logs(::expect_test::Expect {
                position: ::expect_test::Position {
                    file: "salsa-2022-tests/tests/hello_world.rs",
                    line: 104u32,
                    column: 20u32,
                },
                data: r#"
        []"#,
                indent: true,
            });
        }
    }
    mod init {
        pub fn init() {
            {
                let _ = ::env_logger::builder().is_test(true).try_init();
            }
        }
    }
    init::init();
    test_impl()
}
#[rustc_main]
#[no_coverage]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&execute, &red_herring])
}
