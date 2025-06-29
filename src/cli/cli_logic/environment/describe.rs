use crate::app::app::App;
use crate::app::files::environment::OS_ENV_VARS;

impl App<'_> {
    pub fn cli_describe_env(&mut self, env_index: usize, os_vars: bool) -> anyhow::Result<()> {
        let local_env = self.get_env_as_local_from_index(env_index).unwrap();

        {
            let env = local_env.read();

            println!("name: {}", env.name);
            println!("values:");

            for (key, value) in &env.values {
                println!("\t{key}: {value}");
            }

            if os_vars {
                println!("os vars:");
                for (key, values) in OS_ENV_VARS.iter() {
                    println!("\t{key}: {values}");
                }
            }
        }

        Ok(())
    }
}