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
                Ok(
                    $service_fn( $( $arg ),* )
                        .await
                        .map_err(handle_db_err)?
                )
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
                Ok(
                    $service_fn( $( $arg ),* )
                        .await
                        .map_err(handle_db_err)?
                )
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
                Ok(
                    $service_fn( $( $arg ),* )
                        .await
                        .map_err(handle_db_err)?
                )
            }
        )*
    };
}
