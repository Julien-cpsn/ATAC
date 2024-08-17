use crate::app::app::App;

impl App<'_> {
    pub fn cli_describe_env(&mut self, env_index: usize) -> anyhow::Result<()> {
        let local_env = self.get_env_as_local_from_index(env_index).unwrap();

        {
            let env = local_env.read();

            println!("name: {}", env.name);
            println!("values:");

            for (key, value) in &env.values {
                println!("\t{key}: {value}");
            }
        }

        Ok(())
    }
}