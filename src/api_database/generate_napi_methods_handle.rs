#[macro_export]
macro_rules! generate_napi_methods {
    (
        $namespace:expr,
        $(
            $fn_name:ident ( $( $arg:ident : $arg_ty:ty ),* )
            -> $ret:ty
            => $service_fn:path
        ),* $(,)?
    ) => {
        $(
            #[napi(namespace = $namespace)]
            pub async fn $fn_name( $( $arg : $arg_ty ),* ) -> Result<$ret> {
                $service_fn( $( $arg ),* )
                    .await
                    .map_err(handle_db_err)
            }
        )*
    };
}

#[macro_export]
macro_rules! generate_napi_u32_methods {
    (
        $namespace:expr,
        $(
            $fn_name:ident ( $( $arg:ident : $arg_ty:ty ),* )
            -> $ret:ty
            => $service_fn:path
        ),* $(,)?
    ) => {
        $(
            #[napi(namespace = $namespace)]
            pub async fn $fn_name( $( $arg : $arg_ty ),* ) -> Result<u32> {
                $service_fn( $( $arg ),* )
                    .await
                    .map_err(handle_db_err)
            }
        )*
    };
}

#[macro_export]
macro_rules! generate_napi_i32_methods {
    (
        $namespace:expr,
        $(
            $fn_name:ident ( $( $arg:ident : $arg_ty:ty ),* )
            -> $ret:ty
            => $service_fn:path
        ),* $(,)?
    ) => {
        $(
            #[napi(namespace = $namespace)]
            pub async fn $fn_name( $( $arg : $arg_ty ),* ) -> Result<i32> {
                $service_fn( $( $arg ),* )
                    .await
                    .map_err(handle_db_err)
            }
        )*
    };
}
