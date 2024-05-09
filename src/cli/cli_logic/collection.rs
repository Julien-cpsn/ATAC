use crate::app::app::App;
use crate::models::collection::Collection;

impl App<'_> {
    pub fn list_collections(&mut self, with_request_names: bool) -> anyhow::Result<()> {
        for collection in &self.collections {
            print_collection(collection, with_request_names);
        }

        Ok(())
    }

    pub fn describe_collection(&mut self, collection_index: usize) -> anyhow::Result<()> {
        let collection = &self.collections[collection_index];

        print_collection(&collection, true);
        
        Ok(())
    }
    
    pub fn cli_delete_collection(&mut self, collection_name: &str) -> anyhow::Result<()> {
        let collection_index = self.find_collection(collection_name)?;

        self.delete_collection(collection_index);
        
        Ok(())
    }
}

fn print_collection(collection: &Collection, with_request_names: bool) {
    println!("{}", collection.name);

    if with_request_names {
        for request in &collection.requests {
            let local_request = request.read();
            println!("\t{}", local_request.name);
        }
    }
}