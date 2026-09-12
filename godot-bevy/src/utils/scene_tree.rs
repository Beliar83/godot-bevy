use godot::classes::Window;
use godot::prelude::*;

pub trait SceneTreeExtensions {
    /// Returns the root of the scene tree as an Option, regardless of the Godot version.
    /// If you do not need to stay compatible with Godot versions prior to 4.7, you can use the `get_root()` method directly, which does not return an Option.
    /// The return could never be None, as the root is always present in the scene tree, but prior to Godot 4.7, it was not indicated in the api description.
    fn get_root_as_option(&self) -> Option<Gd<Window>>;

    /// Returns the root of the scene tree, expecting it to exist.
    /// If you do not need to stay compatible with Godot versions prior to 4.7, you can use the `get_root()` method directly.
    /// The return could never be None, as the root is always present in the scene tree, but prior to Godot 4.7, it was not indicated in the api description.
    fn get_root_expect(&self) -> Gd<Window>;
}

#[cfg(not(any(
    feature = "api-4-7",
    feature = "api-custom",
    feature = "api-custom-json",
)))]
impl SceneTreeExtensions for Gd<SceneTree> {
    #[inline]
    fn get_root_as_option(&self) -> Option<Gd<Window>> {
        self.get_root()
    }

    #[inline]
    fn get_root_expect(&self) -> Gd<Window> {
        self.get_root().expect("Root should exist")
    }
}

#[cfg(any(
    feature = "api-4-7",
    feature = "api-custom",
    feature = "api-custom-json",
))]
impl SceneTreeExtensions for Gd<SceneTree> {
    #[inline]
    fn get_root_as_option(&self) -> Option<Gd<Window>> {
        Some(self.get_root())
    }

    #[inline]
    fn get_root_expect(&self) -> Gd<Window> {
        self.get_root()
    }
}
