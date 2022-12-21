use std::str::FromStr;
use std::{env, io};

use anyhow::Result;
use kube::config::{AuthInfo, Kubeconfig, NamedAuthInfo};
use secrecy::Secret;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut buf = String::new();
    let token = if args.len() > 1 {
        args[1].trim()
    } else {
        let stdin = io::stdin();
        stdin.read_line(&mut buf)?;
        buf.trim()
    };

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

#[cfg(test)]
mod tests {
    // TODO: impl test
}
