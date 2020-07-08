extern crate regex;

use std::any;
#[allow(unused_imports)]
use std::borrow::Borrow;
use std::ops;

#[derive(Debug)]
struct Foo;

impl Foo {
    #[allow(dead_code)]
    fn get_type_name(&self) -> String {
        get_type_name(&self)
    }
}

impl AsRef<Foo> for Foo {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl From<Bar> for Foo {
    fn from(_: Bar) -> Self {
        Foo
    }
}

impl From<&Bar> for &Foo {
    fn from(_: &Bar) -> Self {
        &Foo
    }
}

struct FooBox(Foo);

impl ops::Deref for FooBox {
    type Target = Foo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Foo> for FooBox {
    fn as_ref(&self) -> &Foo {
        self
    }
}

#[derive(Clone, Copy)]
struct Bar;

impl Bar {
    #[allow(dead_code)]
    fn get_type_name_once(self) -> String {
        get_type_name(&self)
    }
}

struct BarBox(Bar);

impl ops::Deref for BarBox {
    type Target = Bar;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
fn get_type_name_by_foo(foo: &Foo) -> String {
    foo.get_type_name()
}

#[allow(dead_code)]
fn get_type_name_once_by_bar(bar: Bar) -> String {
    bar.get_type_name_once()
}

#[allow(dead_code)]
fn get_type_name_once_by_bar_as_ref(bar: &Bar) -> String {
    bar.get_type_name_once()
}

#[allow(dead_code)]
fn get_type_name_as_ref_foo<T: AsRef<Foo>>(t: &T) -> String {
    get_type_name(&t.as_ref())
}

#[allow(dead_code)]
fn get_type_name<T>(_: &T) -> String {
    regex::Regex::new(r"([a-z]|_)*?::")
        .unwrap()
        .replace_all(&format!("{}", any::type_name::<T>()), "")
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_call_method() {
        let value = Foo;
        assert_eq!("Foo", get_type_name(&value));
        assert_eq!("&Foo", value.get_type_name());

        let reference = &value;
        assert_eq!("&Foo", get_type_name(&reference));
        assert_eq!("&Foo", reference.get_type_name());

        let bar_value = Bar;
        let bar_reference = &bar_value;
        assert_eq!("&Bar", get_type_name(&bar_reference));
        assert_eq!("Bar", bar_reference.get_type_name_once());

        let reference_of_reference = &reference;
        assert_eq!("&&Foo", get_type_name(&reference_of_reference));
        assert_eq!("&Foo", reference_of_reference.get_type_name());

        let boxed = Box::new(value);
        assert_eq!("Box<Foo>", get_type_name(&boxed));
        assert_eq!("&Foo", boxed.get_type_name());

        let foo_box = FooBox(Foo);
        assert_eq!("FooBox", get_type_name(&foo_box));
        assert_eq!("&Foo", foo_box.get_type_name());
    }

    #[test]
    fn on_derefer() {
        let boxed = Box::new(Foo);
        assert_eq!("Box<Foo>", get_type_name(&boxed));
        assert_eq!("Foo", get_type_name(&*boxed));

        let foo_box = FooBox(Foo);
        assert_eq!("FooBox", get_type_name(&foo_box));
        assert_eq!("Foo", get_type_name(&*foo_box));
        assert_eq!("&Foo", get_type_name_by_foo(&foo_box));

        let bar_box = BarBox(Bar);
        assert_eq!("BarBox", get_type_name(&bar_box));
        assert_eq!("Bar", get_type_name(&*bar_box));
        assert_eq!("Bar", get_type_name_once_by_bar(*bar_box));
        assert_eq!("Bar", get_type_name_once_by_bar_as_ref(&bar_box));
    }

    #[test]
    fn as_ref() {
        let value = Foo;
        assert_eq!("Foo", get_type_name(&value));
        assert_eq!("&Foo", get_type_name_as_ref_foo(&value));

        let foo_box = FooBox(value);
        assert_eq!("FooBox", get_type_name(&foo_box));
        assert_eq!("&Foo", get_type_name_as_ref_foo(&foo_box));
    }

    #[test]
    fn on_borrow() {
        let value = Foo;
        assert_eq!("Foo", get_type_name(&value));
        assert_eq!("&Foo", get_type_name(&value.borrow()));
    }

    #[test]
    fn from() {
        let original_value = Bar;
        assert_eq!("Bar", get_type_name(&original_value));
        assert_eq!("&Foo", get_type_name_by_foo(From::from(&original_value)));
        assert_eq!("&Foo", get_type_name_by_foo(&original_value.into()));
    }
}
