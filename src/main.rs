use manifest_dir_macros::directory_relative_path;
use stardust_xr_molecules::fusion::client::Client;
pub mod orbital;

use orbital::Orbital;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (client, event_loop) = Client::connect_with_async_loop()
        .await
        .expect("Unable to connect to server");
    client.set_base_prefixes(&[directory_relative_path!("res")]);

    //Code starts here
    let mut root = Orbital::new(&client);
    let _root_wrapper = client.wrap_root(root);

    tokio::select! {
        e = tokio::signal::ctrl_c() => e.unwrap(),
        e = event_loop => e.unwrap().unwrap(),
    }
}
