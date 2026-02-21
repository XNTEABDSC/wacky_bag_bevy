
#[macro_export]
macro_rules! query_get_or_err_return {
    ($query:ident,$entity:ident) => {
        {
            let Ok(res)=$query.get($entity) else {
                bevy::log::tracing::error!("entity {} not found in {}",$entity,core::any::type_name_of_val(&$query));
                return;
            };
            res
        }
    };
}