use std::collections::HashMap;
use std::process::Command;
use std::time::Duration;
use axum::extract::{State,Json};
use axum::response::Redirect;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use std::sync::Arc;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub busy: bool,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestBody {
    // Define the structure of your request body here
    // Example:
    password: String,
    screen_width: String,
    screen_height: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerMap(pub HashMap<String, Server>);

pub async fn start_container(
    State(server_map): State<Arc<Mutex<ServerMap>>>,
    Json(body): Json<RequestBody>,
) -> Redirect {
    let server_map_arc = server_map.clone();
    let mut server_map = server_map.lock().await;
    for (server_name, server) in server_map.0.iter_mut() {
        if !server.busy {
            server.busy = true;
            let port = server.port;
            let server_name_clone1 = server_name.clone();
            let server_name_clone2 = server_name.clone();
            let server_map_arc_clone = server_map_arc.clone();

            tokio::spawn(async move {
                let _ = Command::new("sudo")
                    .arg("docker")
                    .arg("stop")
                    .arg(&server_name_clone1)
                    .output();
            
                let _ = Command::new("sudo")
                    .arg("docker")
                    .arg("rm")
                    .arg(&server_name_clone1)
                    .output();
                let build_output = Command::new("sudo")
                    .arg("docker")
                    .arg("build")
                    .arg("-t")
                    .arg("burner-browser")
                    .arg("--build-arg")
                    .arg(format!("PASSWORD={}", body.password))
                    .arg(".") // Ensure the build context is specified
                    .output();

                match build_output {
                    Ok(output) => {
                        if !output.status.success() {
                            eprintln!("Docker build failed: {:?}", output);
                            eprintln!("Build stderr: {:?}", String::from_utf8_lossy(&output.stderr));
                            let mut server_map = server_map_arc_clone.lock().await;
                            if let Some(server) = server_map.0.get_mut(&server_name_clone1) {
                                server.busy = false;
                            }
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to execute Docker build command: {}", e);
                        let mut server_map = server_map_arc_clone.lock().await;
                        if let Some(server) = server_map.0.get_mut(&server_name_clone1) {
                            server.busy = false;
                        }
                        return;
                    }
                }

                println!("Executing start command for {:?}", server_name_clone1);

                let run_output = Command::new("sudo")
                    .arg("docker")
                    .arg("run")
                    .arg("-p")
                    .arg(format!("{}:6080", port))
                    .arg("--name")
                    .arg(&server_name_clone1)
                    .arg("burner-browser")
                    .output();

                match run_output {
                    Ok(output) => {
                        if !output.status.success() {
                            eprintln!("Docker run failed: {:?}", output);
                            eprintln!("Run stderr: {:?}", String::from_utf8_lossy(&output.stderr));
                            let mut server_map = server_map_arc_clone.lock().await;
                            if let Some(server) = server_map.0.get_mut(&server_name_clone1) {
                                server.busy = false;
                            }
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to execute Docker run command: {}", e);
                        let mut server_map = server_map_arc_clone.lock().await;
                        if let Some(server) = server_map.0.get_mut(&server_name_clone1) {
                            server.busy = false;
                        }
                        return;
                    }
                }

                println!("Docker container started successfully for {:?}", server_name_clone1);
            });

            tokio::spawn(async move {
                let start_time = tokio::time::Instant::now();
                while Duration::from_secs(60) > start_time.elapsed() {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                println!("Executing stop command for {:?}", server_name_clone2);

                let stop_output = Command::new("sudo")
                    .arg("docker")
                    .arg("stop")
                    .arg(&server_name_clone2)
                    .output();
                let _ = Command::new("sudo")
                    .arg("docker")
                    .arg("rm")
                    .arg(&server_name_clone2)
                    .output();

                match stop_output {
                    Ok(output) => {
                        if output.status.success() {
                            println!("Docker stop success: {:?}", output);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to execute Docker stop command: {}", e);
                    }
                }

                let mut server_map = server_map_arc.lock().await;
                if let Some(server) = server_map.0.get_mut(&server_name_clone2) {
                    server.busy = false;
                }

                let rm_output = Command::new("sudo")
                    .arg("docker")
                    .arg("rm")
                    .arg(&server_name_clone2)
                    .output();

                match rm_output {
                    Ok(output) => {
                        if !output.status.success() {
                            eprintln!("Docker rm failed: {:?}", output);
                            eprintln!("Remove stderr: {:?}", String::from_utf8_lossy(&output.stderr));
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to execute Docker rm command: {}", e);
                    }
                }
            });
            tokio::time::sleep(Duration::from_secs(5)).await;
            return Redirect::to(format!("http://127.0.0.1/novnc/{}/vnc.html?resize=remote&path=novnc/{}/websockify", server_name,server_name).as_str());
        }
    }
    Redirect::to("/") // Handle case where all servers are busy
}