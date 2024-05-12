use std::{
    any::Any,
    marker::Unsize,
    ops::Deref,
    ptr::{DynMetadata, Pointee},
};

use crate::{
    base::Base,
    inlinebox::InlineBox,
    repr::NarrowRepr,
    smart_pointer::{IsSmartPointer, SmartPointer, SmartRepr},
    ClassValue, Subclass,
};

pub trait CastValue: 'static + Any {
    fn into_leaf(&self) -> fn(SmartPointer<dyn Any>) -> BoxCastObject;
}

impl<T> CastValue for T
where
    T: ClassValue,
    <T as ClassValue>::Dyn: CastTrait,
{
    fn into_leaf(&self) -> fn(SmartPointer<dyn Any>) -> BoxCastObject {
        |ptr| {
            let ptr: SmartPointer<T> = SmartPointer::downcast::<T>(ptr).ok().unwrap();
            let ptr: SmartPointer<T::Dyn> = ptr;
            let ptr: InlineBox<SmartPointer<T::Dyn>, _> =
                InlineBox::<SmartPointer<T::Dyn>, _>::new(ptr);
            let ptr: InlineBox<dyn CastObject, _> = ptr.unsize();
            ptr
        }
    }
}

pub trait CastTrait {
    fn into_parent_object(&self) -> fn(BoxCastObject) -> Result<BoxCastObject, BoxCastObject>;
}

impl<T: ?Sized> CastTrait for T
where
    T: Subclass,
    T::Parent: CastTrait,
{
    fn into_parent_object(&self) -> fn(BoxCastObject) -> Result<BoxCastObject, BoxCastObject> {
        |ptr| {
            let ptr: InlineBox<SmartPointer<T>, _> =
                InlineBox::downcast(ptr.unsize()).ok().unwrap();
            let ptr: SmartPointer<T> = ptr.into_inner();
            let ptr: SmartPointer<T::Parent> = ptr;
            let ptr: InlineBox<SmartPointer<T::Parent>, _> = InlineBox::new(ptr);
            let ptr: InlineBox<dyn CastObject, _> = ptr.unsize();
            Ok(ptr)
        }
    }
}

pub trait CastObject: Any {
    fn into_parent_object(&self) -> fn(BoxCastObject) -> Result<BoxCastObject, BoxCastObject>;
}

impl<T: 'static + Deref> CastObject for T
where
    T::Target: CastTrait,
{
    fn into_parent_object(&self) -> fn(BoxCastObject) -> Result<BoxCastObject, BoxCastObject> {
        (**self).into_parent_object()
    }
}
pub type BoxCastObject = InlineBox<dyn CastObject, SmartRepr<NarrowRepr>>;

pub trait Cast<O: 'static + Sized>: Sized {
    fn downcast_trait(self) -> Option<O>;
}

// pub fn coerce_unsized<
//     'a,
//     A: ?Sized + 'static + Pointee<Metadata = DynMetadata<A>>,
//     B: ?Sized + 'static + Pointee<Metadata = DynMetadata<B>> + CastTrait,
// >(
//     this: StackBox<'a, dyn CastObject>,
// ) -> StackBox<'a, dyn CastObject>
// where
//     A: Unsize<B>,
// {
//     let this = this as StackBox<'a, dyn Any>;
//     let this = match this.downcast::<Rc<A>>() {
//         Ok(this) => {
//             let (this, space) = this.into_inner_with();
//             let this: Rc<B> = this;
//             let this: StackBox<Rc<B>> = StackBox::new(this, space);
//             let this: StackBox<dyn CastObject> = this;
//             return this;
//         }
//         Err(this) => this,
//     };
//     let this = match this.downcast::<Arc<A>>() {
//         Ok(this) => {
//             let (this, space) = this.into_inner_with();
//             let this: Arc<B> = this;
//             let this: StackBox<Arc<B>> = StackBox::new(this, space);
//             let this: StackBox<dyn CastObject> = this;
//             return this;
//         }
//         Err(this) => this,
//     };
//     match this.downcast::<Box<A>>() {
//         Ok(this) => {
//             let (this, space) = this.into_inner_with();
//             let this: Box<B> = this;
//             let this: StackBox<Box<B>> = StackBox::new(this, space);
//             let this: StackBox<dyn CastObject> = this;
//             return this;
//         }
//         Err(this) => this,
//     };
//     panic!("Did not find the expected trait object");
// }

impl<P1, P2: 'static> Cast<P2> for P1
where
    P1: IsSmartPointer,
    P1::Target: CastValue + Unsize<dyn Any>,
    P2: IsSmartPointer<Kind = P1::Kind>,
    <P1 as Deref>::Target: Pointee<Metadata = DynMetadata<<P1 as Deref>::Target>>,
{
    fn downcast_trait(self) -> Option<P2> {
        let into_leaf = self.into_leaf();
        let this: SmartPointer<P1::Target> = SmartPointer::new(self);
        let this: SmartPointer<dyn Any> = this;
        let mut this: BoxCastObject = into_leaf(this);
        loop {
            if (&*this as &dyn Any).is::<SmartPointer<P2::Target>>() {
                return Some(
                    InlineBox::downcast::<SmartPointer<P2::Target>>(this.unsize())
                        .unwrap()
                        .into_inner()
                        .into_smart_pointer()
                        .ok()
                        .unwrap(),
                );
            } else {
                this = match (this.into_parent_object())(this) {
                    Ok(this) => this,
                    Err(this) => {
                        let this = InlineBox::downcast::<SmartPointer<dyn Base>>(this.unsize())
                            .ok()
                            .unwrap();
                        let this = this.into_inner();
                        this.try_drop();
                        return None;
                    }
                }
            }
        }
    }
}
