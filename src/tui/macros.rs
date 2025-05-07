#[macro_export]
macro_rules! get_key_bindings {
    (
        $(#[$enum_meta:meta])*
         $vis:vis enum $name:ident {
            $($variant:ident($type:ty)),* $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        $vis enum $name {
            $($variant($type)),*
        }

        impl $name {
            pub fn get_event_key_bindings(&self) -> &EventKeyBinding {
                match self {
                    $(Self::$variant(event_key_bindings) => event_key_bindings),*
                }
            }
        }
    };
}