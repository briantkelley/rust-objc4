pub use paste;

/// Defines a new Rust type for an Objective-C class defined in an external library and implements
/// all the given class hierarchy traits.
#[macro_export]
macro_rules! extern_class {
    // -kind
    ($library:ident, $vis:vis $($class:ident $($class_interface:lifetime)? $(< $($class_param:ident),+ >)?),+ $(; $($param:ident : $ty:path),+)?) => {
        $crate::extern_class!(@1 $library; framework; $vis $($class $($class_interface)? $(< $($class_param),+ >)?),+ $(; $($param : $ty),+)?);
    };
    // +kind
    ($library:ident, kind = $kind:ident, $vis:vis $($class:ident $($class_interface:lifetime)? $(< $($class_param:ident),+ >)?),+ $(; $($param:ident : $ty:path),+)?) => {
        $crate::extern_class!(@1 $library; $kind; $vis $($class $($class_interface)? $(< $($class_param),+ >)?),+ $(; $($param : $ty),+)?);
    };
    // private impl
    (@1 $library:ident; $kind:ident; $vis:vis $ident:ident $($class_interface:lifetime)? $(< $($class_param:ident),+ >)?, $($super:ident $($super_class_interface:lifetime)? $(< $($super_param:ident),+ >)?),+ $(; $($param:ident : $ty:path),+)?) => {
        $crate::extern_class!(@1 $library; $kind; $vis $ident $($class_interface)? $(< $($class_param),+ >)? $(; $($param : $ty),+)?);
        $crate::extern_class!(@2 $ident $($class_interface)? $(< $($class_param),+ >)?; $($super $($super_class_interface)? $(< $($super_param),+ >)?),+ $(; $($param : $ty),+)?);
    };
    (@1 $library:ident; $kind:ident; $vis:vis $ident:ident $($class_interface:lifetime)? $(< $($class_param:ident),+ >)? $(; $($param:ident : $ty:path),+)?) => {
        core::arch::global_asm!(
            "    .pushsection __DATA,__objc_classrefs,regular,no_dead_strip",
            "    .p2align 3",
            concat!("_", stringify!($ident), "ClassReference:"),
            concat!("    .quad    _OBJC_CLASS_$_", stringify!($ident)),
            "    .popsection",
        );

        $crate::paste::paste! {
            #[link(name = "" $library, kind = "" $kind)]
            extern "C" {
                #[link_name = "OBJC_CLASS_$_" $ident]
                static [< _ $ident Class >]: [< $ident ClassType >];
            }

            #[allow(missing_docs, non_upper_case_globals)]
            $vis static [< $ident Class >]: &[< $ident ClassType >] = unsafe { &[< _ $ident Class >] };

            #[allow(missing_copy_implementations, missing_docs)]
            #[repr(transparent)]
            $vis struct [< $ident ClassType >] (
                $crate::objc_class,
            );

            impl core::fmt::Debug for [< $ident ClassType >] {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    self.0.fmt(f)
                }
            }
        }

        $crate::extern_class!(@3 $ident, $ident $($class_interface)?);

        #[allow(missing_copy_implementations, missing_docs)]
        #[repr(C)]
        $vis struct $ident $(< $($param : $ty),+ >)? (
            [u8; core::mem::size_of::<usize>()],
            $($(core::marker::PhantomData<$param>,)+)?
        );

        impl $(< $($param),+ >)? core::fmt::Debug for $ident $(< $($param),+ >)?
        $(where $($param : $ty),+)?
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let obj = $crate::Object::as_ptr(self);
                // SAFETY: `obj` is derived from a reference so it is guaranteed to be a valid
                // pointer to an Objective-C object.
                unsafe { &*obj }.fmt(f)
            }
        }

        $crate::paste::paste! {
            impl $(< $($param),+ >)? $crate::Object for $ident $(< $($param),+ >)?
            $(where $($param : $ty),+)?
            {}

            impl $(< $($param),+ >)? $crate::NSObjectProtocol for $ident $(< $($param),+ >)?
            $(where $($param : $ty),+)?
            {}

            impl $(< $($param),+ >)? [< $ident Interface >] for $ident $(< $($param),+ >)?
            $(where $($param : $ty),+)?
            {
                $($(type $class_param = $class_param;)+)?
            }
        }
    };
    (@2 $ident:ident $($class_interface:lifetime)? $(< $($class_param:ident),+ >)?; $super:ident $($super_class_interface:lifetime)? $(< $($super_param:ident),+ >)? $(; $($param:ident : $ty:path),+)?) => {
        $crate::extern_class!(@3 $ident, $super $($super_class_interface)? $($($param : $ty),+)?);
        $crate::paste::paste! {
            impl $(< $($param),+ >)? [< $super Interface >] for $ident $(< $($param),+ >)?
            $(where $($param : $ty),+)?
            {
                $($(type $super_param = $super_param;)+)?
            }
        }
    };
    (@2 $ident:ident $($class_interface:lifetime)? $(< $($class_param:ident),+ >)?; $super:ident $($super_class_interface:lifetime)? $(< $($super_param:ident),+ >)?, $($ancestors:ident $($ancestor_class_interface:lifetime)? $(< $($ancestor_param:ident),+ >)?),+ $(; $($param:ident : $ty:path),+)?) => {
        $crate::extern_class!(@2 $ident $($class_interface)? $(< $($class_param),+ >)?; $super $($super_class_interface)? $(< $($super_param),+ >)? $(; $($param : $ty),+)?);
        $crate::extern_class!(@2 $ident $($class_interface)? $(< $($class_param),+ >)?; $($ancestors $($ancestor_class_interface)? $(< $($ancestor_param),+ >)?),+ $(; $($param : $ty),+)?);
    };
    (@3 $ident:ident, $super:ident $($($param:ident : $ty:path),+)?) => {};
    (@3 $ident:ident, NSObject 'cls) => {
        $crate::paste::paste! {
            impl $crate::NSObjectClassInterface for [< $ident ClassType >] {
                type Instance = $ident;
            }
        }
    };
    (@3 $ident:ident, NSObject 'cls $($param:ident : $ty:path),+) => {
        $crate::paste::paste! {
            impl $crate::NSObjectClassInterface for [< $ident ClassType >] {
                // Use `id` for types with generic parameters. Otherwise, the class type would
                // require generic parameters to specify the generic types on the associated type,
                // which creates *n* class types where only 1 exists.
                type Instance = $crate::NSObject;
            }
        }
    };
    (@3 $ident:ident, $super:ident 'cls $($($param:ident : $ty:path),+)?) => {
        $crate::paste::paste! {
            impl [< $super ClassInterface >] for [< $ident ClassType >] {}
        }
    };
}

/// A macro to type cast `objc_msgSend` with the correct return type and argument types so the
/// compiler can pass the arguments as required by the ABI.
#[macro_export]
macro_rules! msg_send {
    ($ret:ty $(, $ty:ty)*) => {
        // SAFETY: Assume the user of the macro provided the correct return type, receiver type,
        // selector instance, and argument types.
        unsafe {
            let untyped: unsafe extern "C" fn() = $crate::objc_msgSend;
            core::mem::transmute::<
                _,
                extern "C" fn($crate::id, *const u8 $(, $ty)*) -> $ret,
            >(untyped)
        }
    };
}

/// A convenience macro to wrap the read of a selector symbol in an `unsafe` block.
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! sel {
    [$cmd:ident] => {
        // SAFETY: Rust code never reads through the reference. The reference is passed to the
        // Objective-C runtime, which is the owner of the data type.
        unsafe { crate::sel::$cmd }
    }
}

/// Create a symbol for a selector. Requires providing the literal spelling as used by the runtime.
#[macro_export]
macro_rules! selector {
    ($ident:ident = $name:literal) => {
        core::arch::global_asm!(
            "    .pushsection __TEXT,__objc_methname,cstring_literals",
            concat!("l_SELECTOR_NAME_", stringify!($ident), ":"),
            concat!("    .asciz   \"", $name, "\""),
            "",
            "    .section     __DATA,__objc_selrefs,literal_pointers,no_dead_strip",
            "    .p2align 3",
            concat!("_SELECTOR_", stringify!($ident), ":"),
            concat!("    .quad    l_SELECTOR_NAME_", stringify!($ident)),
            "    .popsection",
        );

        extern "C" {
            $crate::paste::paste! {
                #[link_name = "SELECTOR_" $ident]
                pub(super) static $ident: *const u8;
            }
        }
    };
}
