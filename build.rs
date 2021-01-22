fn main() {
    println!("cargo:rerun-if-changed=client/public/controller.html");

    println!("cargo:rerun-if-changed=client/src/css/controller.css");

    println!("cargo:rerun-if-changed=client/src/pages/controller/index.js");

    println!("cargo:rerun-if-changed=client/webpack.config.js");

    let build = match std::env::var("PROFILE").unwrap().as_str() {
        "debug" => "build-dev",
        "release" => "build-prod",
        _ => panic!()
    };

    let status = std::process::Command::new("npm")
        .arg("run")
        .arg(build)
        .current_dir("client")
        .status()
        .unwrap();
    assert!(status.success());
}
