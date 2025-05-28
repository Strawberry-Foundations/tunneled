use stblib::external::plugin::{Plugin, PluginProperties};

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn execute(&self, args: &[String]) {
        let command = args.first().map(|s| s.as_str()).unwrap_or("");
        
        match command {
            "test" => {
                println!("Example tunneled plugin")
            },
            "foo" => {
                println!("Bar!")
            },
            "" => {
                println!("No command provided");
                self.help();
            },
            _ => {
                println!("Unknown command: '{}'", command);
                self.help();
            }
        }
    }

    fn help(&self) {
        println!("Example Plugin Help:");
        println!("  test  - Run test command");
        println!("  foo   - Print 'Bar!'");
        println!("  help  - Show this help message");
    }
}

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn create_plugin() -> (Box<dyn Plugin>, PluginProperties) {
    const PROPERTIES: PluginProperties = PluginProperties {
        name: "Example Plugin",
        id: "example-plugin",
        package_id: "com.example.exampleplugin",
        version: env!("CARGO_PKG_VERSION"),
        library_version: stblib::VERSION,
    };

    (Box::new(ExamplePlugin), PROPERTIES)
}