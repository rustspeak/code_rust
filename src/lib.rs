use yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use anyhow::Result;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Project {
    id: u64,
    name: String,
    description: String,
}

#[function_component(App)]
fn app() -> Html {
    let project = use_state(|| Project {
        id: 1,
        name: "Projet Test".to_string(),
        description: "Description du projet".to_string(),
    });

    let create_nft = {
        let project = project.clone();
        Callback::from(move |_| {
            let project = project.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Err(e) = create_nft(project.as_ref().clone()).await {
                    log::error!("Error creating NFT: {:?}", e);
                }
            });
        })
    };

    html! {
        <div>
            <h1>{ "Aptos NFT Project" }</h1>
            <button onclick={create_nft}>{ "Create NFT" }</button>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}

async fn create_nft(project: Project) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8080/create_nft")
        .json(&project)
        .send()
        .await?;
    if res.status().is_success() {
        log::info!("NFT created successfully!");
    } else {
        log::error!("Failed to create NFT");
    }
    Ok(())
}
