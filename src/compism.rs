use route::RoutePoint;
use plugin::Plugin;
use std::path::Path;
use std::default::Default;
use error;
use error::Result;

pub struct Compism {
    root: RoutePoint,
}

impl Default for Compism {
    fn default() -> Self {
        Compism {
            root: RoutePoint::default()
        }
    }
}

impl Compism {
    /// Create a new Compism structure. This is the default Compism structure with only root
    /// inside. Root can not be re-mount, every plugin should be under root.
    /// # Example
    /// ```
    /// use compism::Compism;
    ///
    /// let com = Compism::new();
    /// let com_default = Compism::default();
    /// // These two are actually the same
    /// ```
    pub fn new() -> Self {
        Compism::default()
    }

    /// Mount a plugin at a routepoint as a sub-route, `path` is the path to be mount,
    /// `lib_path` is the library(dll or so file) containing the plugin.
    pub fn mount(&mut self, path: &Path, lib_path: &Path) -> Result<()> {
        if !path.is_absolute() || path == Path::new("/") {
            return Err(error::CompismErrorKind::MountPathInvalid)?;
        }

        let parent = path.parent()
            .ok_or(Err(error::CompismErrorKind::MountPathInvalid)?)?;
        let plugin = Plugin::new(lib_path)?;
        let name = path.file_name()
            .ok_or(Err(error::CompismErrorKind::MountPathInvalid)?)?
            .to_str().ok_or(Err(error::CompismErrorKind::MountPathInvalid)?)?;
        // TODO remove this new block only for lifetime, borrowed in `find_route_mut` cannot extend
        // to whole Compism struct. Should be limited in this function only.
        // but how?
        {
            let root: &mut RoutePoint = &mut self.root;
            let parent_route = RoutePoint::find_route_mut(root, parent)?;
            let new_route = RoutePoint::new(name, plugin);
            parent_route.register_subroute(new_route);
        }
        Ok(())
    }

    pub fn find_route_mut(&mut self, path: &Path) -> Result<&mut RoutePoint> {
        let parent = path.parent()
            .ok_or(Err(error::CompismErrorKind::InvalidRoutePath)?)?;
        RoutePoint::find_route_mut(&mut self.root, parent)
    }
}
