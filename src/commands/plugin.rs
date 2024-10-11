use stblib::external::plugin::{Plugin, PluginProperties};

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn execute(&self, args: &[String]) {
        match args.first().unwrap().as_str() {
            "test" => {
                println!("Example plugin")
            },
            "foo" => {
                println!("Bar!")
            }
            _ => self.help()
        }
    }

    fn help(&self) {
        println!("Example help message")
    }
}

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn create_plugin() -> (Box<dyn Plugin>, PluginProperties) {
    let properties: PluginProperties = PluginProperties {
        name: "Example Plugin",
        id: "example-plugin",
        package_id: "com.example.exampleplugin",
        version: env!("CARGO_PKG_VERSION"),
        library_version: stblib::VERSION,
    };

    (Box::new(ExamplePlugin), properties)
}