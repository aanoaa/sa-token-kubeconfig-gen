use std::str::FromStr;
use std::{env, io};

use anyhow::Result;
use k8s_openapi::api::core::v1::Secret as KubeSecret;
use kube::config::{AuthInfo, Kubeconfig, NamedAuthInfo};
use kube::{Api, Client};
use secrecy::Secret;
use thiserror::Error;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut buf = String::new();
    let token = match args.len() {
        x if x > 2 => token_from_secret(args[1].trim(), args[2].trim()).await?,
        2 => args[1].clone(),
        _ => {
            let stdin = io::stdin();
            stdin.read_line(&mut buf)?;
            buf
        }
    };
    let token = token.trim();
    let kubeconfig = Kubeconfig::read()?;
    let mut ctx = kubeconfig.current_context.expect("current context not set");
    let contexts = kubeconfig.contexts;
    let mut context = contexts
        .iter()
        .find(|c| c.name.eq(&ctx))
        .expect("not found context")
        .clone();

    let name = context.context.as_ref().map(|c| c.cluster.clone()).unwrap();
    let clusters = kubeconfig.clusters;
    let cluster = clusters
        .iter()
        .find(|c| c.name.eq(&name))
        .expect("not found cluster")
        .clone();

    let auth_info = NamedAuthInfo {
        name: "deployment".to_string(),
        auth_info: Some(AuthInfo {
            token: Some(Secret::from_str(token).unwrap()),
            ..Default::default()
        }),
    };

    ctx = format!("deployment@{}", cluster.name);
    context.name = ctx.clone();

    context
        .context
        .as_mut()
        .map(|c| c.user = "deployment".to_string())
        .unwrap();

    let new_config = Kubeconfig {
        clusters: vec![cluster],
        contexts: vec![context],
        current_context: Some(ctx),
        auth_infos: vec![auth_info],
        ..Default::default()
    };

    println!("{}", serde_yaml::to_string(&new_config).unwrap());
    Ok(())
}

#[derive(Error, Debug)]
enum TokenError {
    #[error("empty token")]
    Empty,
}

async fn token_from_secret(namespace: &str, name: &str) -> Result<String> {
    let client = Client::try_default().await?;
    let api: Api<KubeSecret> = Api::namespaced(client, namespace);
    let secret = api.get(name).await?;
    let mut token = String::new();
    if let Some(data) = secret.data {
        if let Some(b) = data.get("token") {
            token = std::str::from_utf8(&b.0)?.to_string();
        }
    }

    if token.is_empty() {
        return Err(TokenError::Empty.into());
    }

    Ok(token)
}

#[cfg(test)]
mod tests {
    // TODO: impl test
}
