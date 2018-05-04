use std::default::Default;
use plugin::Plugin;
use std::path::Path;
use error;
use error::Result;

#[derive(Debug)]
pub struct RoutePoint {
    /// Name of the route point(name of the plugin)
    pub name: String,
    /// Routes under this route point
    pub sub_routes: Vec<RoutePoint>,
    /// Plugin behind the route. Root has no plugin behind, then it is `None`.
    pub plugin: Option<Plugin>,
}

impl RoutePoint {
    /// New route point.
    pub fn new(name: &str, plugin: Plugin) -> Self {
        RoutePoint {
            name: name.to_string(),
            sub_routes: Vec::new(),
            plugin: Some(plugin),
        }
    }

    /// Find a route point according to a path string.
    pub fn find_route_mut<'a>(
        root: &'a mut RoutePoint,
        path: &Path) -> Result<&'a mut Self> 
    {
        if path.is_relative() {
            return Err(error::CompismErrorKind::MountPathInvalid)?;
        }

        let mut cur = root;

        for component in path.iter() {
            if component == Path::new("/") {
                continue
            }
            let s = component.to_str()
                .ok_or(error::CompismErrorKind::InvalidRoutePath)?;
            // A little re-bind to make the borrow stop and we are able
            // to get the `found` into `cur`.
            //
            // If we do something like this:
            // ```
            // cur = cur.subroutes.iter_mut().find(|e| e.name == s)...
            // ```
            // This will annoy borrow checker since `cur` is still borrowed
            // and we are changing it.(Looks strange to me as we are changing
            // the very same variable to another one, previous borrow should
            // actually ends, but borrow checker dosen's think so.
            //
            // So this rebind works as the "rebinding" process definitely 
            // informed borrow checker that the previous binding of the borrow
            // is ended.
            let found = cur;
            let found = found.sub_routes.iter_mut().find(|e| e.name == s)
                .ok_or(error::CompismErrorKind::NoSuchRoutePoint)?;
            cur = found;
        }
        Ok(cur)
    }

    /// Reigster a route to be this route's subroute
    pub fn register_subroute(&mut self, route: RoutePoint) {
        self.sub_routes.push(route);
    }
}

impl Default for RoutePoint {
    fn default() -> Self {
        RoutePoint {
            name: "/".to_string(),
            sub_routes: Vec::new(),
            plugin: None,
        }
    }
}

#[test]
fn test_register_subroute() {
    // Tests on unix path, linux and macos use unix path
    let mut root = RoutePoint::default();
    let mut fake_plugin = RoutePoint::default();
    fake_plugin.name = "fake".to_string();
    root.register_subroute(fake_plugin);
    {
        let found = RoutePoint::find_route_mut(&mut root, Path::new("/fake/"));
        assert_eq!(found.expect("should be found").name, "fake");
    }

    {
        let found = RoutePoint::find_route_mut(&mut root, Path::new("/fake"));
        assert_eq!(found.expect("should be found").name, "fake");
    }
}
