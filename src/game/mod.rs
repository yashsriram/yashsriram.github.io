pub mod threed;
pub mod twod;

#[macro_export]
macro_rules! register_and_draw_bodies {
    (
        $app:ident,
        {
            $resource:ident -> $drawing:ident,
            $($next_resource:ident -> $next_drawing:ident,)*
        } ) => {

        $app.init_resource::<$resource>();
        $app.add_systems(
            Update,
            $drawing,
        );

        register_and_draw_bodies!($app, { $($next_resource -> $next_drawing,)* })
    };
    ($app:ident, {}) => {};
}
