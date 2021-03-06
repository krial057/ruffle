use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::function::Executable;
use crate::avm1::property::Attribute;
use crate::avm1::sound_object::SoundObject;
use crate::avm1::{Object, ObjectPtr, ScriptObject, TObject, Value};
use crate::context::UpdateContext;
use crate::display_object::DisplayObject;
use enumset::EnumSet;
use gc_arena::{Collect, GcCell, MutationContext};

use std::borrow::Cow;
use std::fmt;

/// A SharedObject
#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct SharedObject<'gc>(GcCell<'gc, SharedObjectData<'gc>>);

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct SharedObjectData<'gc> {
    /// The underlying script object.
    base: ScriptObject<'gc>,

    /// The local name of this shared object
    name: Option<String>,
    // In future this will also handle remote SharedObjects
}

impl fmt::Debug for SharedObject<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let this = self.0.read();
        f.debug_struct("SharedObject")
            .field("name", &this.name)
            .finish()
    }
}

impl<'gc> SharedObject<'gc> {
    pub fn empty_shared_obj(
        gc_context: MutationContext<'gc, '_>,
        proto: Option<Object<'gc>>,
    ) -> Self {
        SharedObject(GcCell::allocate(
            gc_context,
            SharedObjectData {
                base: ScriptObject::object(gc_context, proto),
                name: None,
            },
        ))
    }

    pub fn set_name(&self, gc_context: MutationContext<'gc, '_>, name: String) {
        self.0.write(gc_context).name = Some(name);
    }

    pub fn get_name(&self) -> String {
        self.0
            .read()
            .name
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "".to_string())
    }

    fn base(self) -> ScriptObject<'gc> {
        self.0.read().base
    }
}

impl<'gc> TObject<'gc> for SharedObject<'gc> {
    fn get_local(
        &self,
        name: &str,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
        this: Object<'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        self.base().get_local(name, activation, context, this)
    }

    fn set(
        &self,
        name: &str,
        value: Value<'gc>,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
    ) -> Result<(), Error<'gc>> {
        self.base().set(name, value, activation, context)
    }
    fn call(
        &self,
        name: &str,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
        this: Object<'gc>,
        base_proto: Option<Object<'gc>>,
        args: &[Value<'gc>],
    ) -> Result<Value<'gc>, Error<'gc>> {
        self.base()
            .call(name, activation, context, this, base_proto, args)
    }

    fn call_setter(
        &self,
        name: &str,
        value: Value<'gc>,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
    ) -> Option<Executable<'gc>> {
        self.base().call_setter(name, value, activation, context)
    }

    #[allow(clippy::new_ret_no_self)]
    fn new(
        &self,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
        _this: Object<'gc>,
        _args: &[Value<'gc>],
    ) -> Result<Object<'gc>, Error<'gc>> {
        Ok(SharedObject::empty_shared_obj(
            context.gc_context,
            Some(activation.avm.prototypes.shared_object),
        )
        .into())
    }

    fn delete(
        &self,
        activation: &mut Activation<'_, 'gc>,
        gc_context: MutationContext<'gc, '_>,
        name: &str,
    ) -> bool {
        self.base().delete(activation, gc_context, name)
    }

    fn proto(&self) -> Option<Object<'gc>> {
        self.base().proto()
    }

    fn set_proto(&self, gc_context: MutationContext<'gc, '_>, prototype: Option<Object<'gc>>) {
        self.base().set_proto(gc_context, prototype);
    }

    fn define_value(
        &self,
        gc_context: MutationContext<'gc, '_>,
        name: &str,
        value: Value<'gc>,
        attributes: EnumSet<Attribute>,
    ) {
        self.base()
            .define_value(gc_context, name, value, attributes)
    }

    fn set_attributes(
        &mut self,
        gc_context: MutationContext<'gc, '_>,
        name: Option<&str>,
        set_attributes: EnumSet<Attribute>,
        clear_attributes: EnumSet<Attribute>,
    ) {
        self.base()
            .set_attributes(gc_context, name, set_attributes, clear_attributes)
    }

    fn add_property(
        &self,
        gc_context: MutationContext<'gc, '_>,
        name: &str,
        get: Executable<'gc>,
        set: Option<Executable<'gc>>,
        attributes: EnumSet<Attribute>,
    ) {
        self.base()
            .add_property(gc_context, name, get, set, attributes)
    }

    fn add_property_with_case(
        &self,
        activation: &mut Activation<'_, 'gc>,
        gc_context: MutationContext<'gc, '_>,
        name: &str,
        get: Executable<'gc>,
        set: Option<Executable<'gc>>,
        attributes: EnumSet<Attribute>,
    ) {
        self.base()
            .add_property_with_case(activation, gc_context, name, get, set, attributes)
    }

    fn has_property(
        &self,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
        name: &str,
    ) -> bool {
        self.base().has_property(activation, context, name)
    }

    fn has_own_property(
        &self,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
        name: &str,
    ) -> bool {
        self.base().has_own_property(activation, context, name)
    }

    fn has_own_virtual(
        &self,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
        name: &str,
    ) -> bool {
        self.base().has_own_virtual(activation, context, name)
    }

    fn is_property_enumerable(&self, activation: &mut Activation<'_, 'gc>, name: &str) -> bool {
        self.base().is_property_enumerable(activation, name)
    }

    fn get_keys(&self, activation: &mut Activation<'_, 'gc>) -> Vec<String> {
        self.base().get_keys(activation)
    }

    fn as_string(&self) -> Cow<str> {
        Cow::Owned(self.base().as_string().into_owned())
    }

    fn type_of(&self) -> &'static str {
        self.base().type_of()
    }

    fn interfaces(&self) -> Vec<Object<'gc>> {
        self.base().interfaces()
    }

    fn set_interfaces(
        &mut self,
        gc_context: MutationContext<'gc, '_>,
        iface_list: Vec<Object<'gc>>,
    ) {
        self.base().set_interfaces(gc_context, iface_list)
    }

    fn as_script_object(&self) -> Option<ScriptObject<'gc>> {
        Some(self.base())
    }

    fn as_display_object(&self) -> Option<DisplayObject<'gc>> {
        None
    }

    fn as_executable(&self) -> Option<Executable<'gc>> {
        None
    }

    fn as_sound_object(&self) -> Option<SoundObject<'gc>> {
        None
    }

    fn as_shared_object(&self) -> Option<SharedObject<'gc>> {
        Some(*self)
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        self.0.as_ptr() as *const ObjectPtr
    }

    fn length(&self) -> usize {
        self.base().length()
    }

    fn array(&self) -> Vec<Value<'gc>> {
        self.base().array()
    }

    fn set_length(&self, gc_context: MutationContext<'gc, '_>, length: usize) {
        self.base().set_length(gc_context, length)
    }

    fn array_element(&self, index: usize) -> Value<'gc> {
        self.base().array_element(index)
    }

    fn set_array_element(
        &self,
        index: usize,
        value: Value<'gc>,
        gc_context: MutationContext<'gc, '_>,
    ) -> usize {
        self.base().set_array_element(index, value, gc_context)
    }

    fn delete_array_element(&self, index: usize, gc_context: MutationContext<'gc, '_>) {
        self.base().delete_array_element(index, gc_context)
    }
}
