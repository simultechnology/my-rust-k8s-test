use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams},
    Client
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("start!");
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);
    let pod_list = pods.list(&ListParams::default()).await?;
    let names = pod_list.into_iter()
        .map(|pod| pod.metadata.name.unwrap_or("".into()))
        .collect::<Vec<String>>();
    // println!("{names:?}");
    for name in &names {
        println!("{:?}", name);
    }
    Ok(())
}
