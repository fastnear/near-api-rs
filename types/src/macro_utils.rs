#[macro_export]
macro_rules! create_equivalent_types {
    (
        $(
            $(#[$attrs:meta])*
            $vis:vis $type_kind:ident $name:ident
            $body:tt
            as $other_path:path
        );* $(;)?
    ) => {
        $(
            create_equivalent_types!(@single
                $(#[$attrs])*
                $vis $type_kind $name
                $body
                as $other_path
            );
        )*
    };

    (@single
        $(#[$attrs:meta])*
        $vis:vis struct $name:ident
        {
            $(
                $(#[$field_attrs:meta])*
                $field_vis:vis $field:ident: $type:ty
            ),* $(,)?
        }
        as $other_path:path
    ) => {
        $(#[$attrs])*
        $vis struct $name {
            $(
                $(#[$field_attrs])*
                $field_vis $field: $type
            ),*
        }

        impl From<$name> for $other_path {
            fn from(value: $name) -> Self {
                Self {
                    $(
                        $field: value.$field.into()
                    ),*
                }
            }
        }

        impl From<$other_path> for $name {
            fn from(value: $other_path) -> Self {
                Self {
                    $(
                        $field: value.$field.into()
                    ),*
                }
            }
        }
    };

    (@single
        $(#[$attrs:meta])*
        $vis:vis enum $name:ident
        {
            $(
                $(#[$variant_attrs:meta])*
                $variant:ident $(($($tuple_type:ty),* $(,)?))?
                $({
                    $(
                        $field:ident: $field_type:ty
                    ),* $(,)?
                })?
            ),* $(,)?
        }
        as $other_path:path
    ) => {
        $(#[$attrs])*
        $vis enum $name {
            $(
                $(#[$variant_attrs])*
                $variant
                $(($($tuple_type),*))?
                $({
                    $(
                        $field: $field_type
                    ),*
                })?
            ),*
        }

        impl From<$name> for $other_path {
            fn from(value: $name) -> Self {
                match value {
                    $(
                        $name::$variant
                        $(($($tuple_var),*))?
                        $({
                            $(
                                $field: $field_var
                            ),*
                        })? =>
                            Self::$variant
                            $(($($tuple_var),*))?
                            $({
                                $(
                                    $field: $field_var
                                ),*
                            })?
                    ),*
                }
            }
        }

        impl From<$other_path> for $name {
            fn from(value: $other_path) -> Self {
                match value {
                    $(
                        $other_path::$variant
                        $(($($tuple_var),*))?
                        $({
                            $(
                                $field: $field_var
                            ),*
                        })? =>
                            Self::$variant
                            $(($($tuple_var),*))?
                            $({
                                $(
                                    $field: $field_var
                                ),*
                            })?
                    ),*
                }
            }
        }
    };
}
