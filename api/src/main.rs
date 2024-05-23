use actix_web::{web, App, HttpServer};

pub struct Compose {
    pub version: Option<String>,
    pub services: Vec<Service>,
    pub volumes: Vec<Volume>,
    pub networks: Vec<Network>,
    pub environments: Vec<Environment>,
    pub extensions: Vec<Extension>,
}

pub struct Service {
    pub image: String,
    pub container_name: String,
    pub ports: Vec<Port>,
    pub volumes: Vec<Volume>,
    pub networks: Vec<Network>,
    pub depends_on: Vec<Dependency>,
    pub restart: Policy,
    pub environments: Vec<Environment>,
    pub command: Command,
    pub labels: Vec<Label>,
}

pub struct Port {
    pub host: String,
    pub container: String
}

pub struct Dependency {
    pub service: String
}

pub struct Policy {
    pub condition: String
}

pub struct Command {
    pub value: String
}

pub struct Label {
    pub key: String,
    pub value: String
}

pub struct Volume {
    pub title: String,
    pub driver: String
}

pub struct Network {
    pub driver: String,
    pub external: bool
}

pub struct Environment {
    pub title: String,
    pub value: String
}

pub struct Extension {
    pub title: String,
    pub value: String
}


pub fn parse_compose(compose: Compose) -> String {
    let mut compose_str = String::new();
    compose_str.push_str("version: ");
    compose_str.push_str(&compose.version.unwrap());
    compose_str.push_str("\n");

    compose_str.push_str("services:\n");
    for service in compose.services {
        compose_str.push_str("  ");
        compose_str.push_str(&service.container_name);
        compose_str.push_str(":\n");
        compose_str.push_str("    image: ");
        compose_str.push_str(&service.image);
        compose_str.push_str("\n");
        compose_str.push_str("    container_name: ");
        compose_str.push_str(&service.container_name);
        compose_str.push_str("\n");

        compose_str.push_str("    ports:\n");
        if service.ports.len() > 0 {
            for port in service.ports {
                compose_str.push_str("      - ");
                compose_str.push_str(&port.host);
                compose_str.push_str(":");
                compose_str.push_str(&port.container);
                compose_str.push_str("\n");
            }
        }

        compose_str.push_str("    volumes:\n");
    
        if service.volumes.len() > 0 {
            for volume in service.volumes {
                compose_str.push_str("      - ");
                compose_str.push_str(&volume.title);
                compose_str.push_str(":");
                compose_str.push_str(&volume.driver);
                compose_str.push_str("\n");
            }
        }

        if service.depends_on.len() > 0 {
            compose_str.push_str("    depends_on:\n");
            for dependency in service.depends_on {
                compose_str.push_str("      - ");
                compose_str.push_str(&dependency.service);
                compose_str.push_str("\n");
            }
        }

        if service.restart.condition.len() > 0 {
            compose_str.push_str("    restart: ");
            compose_str.push_str(&service.restart.condition);
            compose_str.push_str("\n");
        }

        if service.command.value.len() > 0 {
            compose_str.push_str("    command: ");
            compose_str.push_str(&service.command.value);
            compose_str.push_str("\n");
        }

        if service.labels.len() > 0 {
            compose_str.push_str("    labels:\n");
            for label in service.labels {
                compose_str.push_str("      - ");
                compose_str.push_str(&label.key);
                compose_str.push_str("=");
                compose_str.push_str(&label.value);
                compose_str.push_str("\n");
            }
        }

        if service.environments.len() > 0 {
            compose_str.push_str("    environment:\n");
            for environment in service.environments {
                compose_str.push_str("      - ");
                compose_str.push_str(&environment.title);
                compose_str.push_str("=");
                compose_str.push_str(&environment.value);
                compose_str.push_str("\n");
            }
        }

        if service.networks.len() > 0 {
            compose_str.push_str("    networks:\n");
            for network in service.networks {
                compose_str.push_str("      - ");
                compose_str.push_str(&network.driver);
                compose_str.push_str("\n");
            }
        }

    }

    compose_str.push_str("volumes:\n");
    compose_str.push_str("  ");
    for volume in compose.volumes {
        compose_str.push_str(&volume.title);
        compose_str.push_str(":\n");
        compose_str.push_str("    driver: ");
        compose_str.push_str(&volume.driver);
        compose_str.push_str("\n");
    }

    compose_str.push_str("networks:\n");
    compose_str.push_str("  ");
    for network in compose.networks {
        compose_str.push_str(&network.driver);
        compose_str.push_str(":\n");
        compose_str.push_str("    external: ");
        compose_str.push_str(&network.external.to_string());
        compose_str.push_str("\n");
    }

    compose_str
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/dc").to(|| async { 
                let compose = Compose {
                    version: Some("3.8".to_string()),
                    services: vec![
                        Service {
                            image: "mysql".to_string(),
                            container_name: "mysql".to_string(),
                            ports: vec![
                                Port {
                                    host: "3306".to_string(),
                                    container: "3306".to_string()
                                }
                            ],
                            volumes: vec![
                                Volume {
                                    title: "./mysql".to_string(),
                                    driver: "/var/lib/mysql".to_string()
                                }
                            ],
                            networks: vec![
                                Network {
                                    driver: "web".to_string(),
                                    external: false
                                }
                            ],
                            environments: vec![
                                Environment {
                                    title: "MYSQL_ROOT_PASSWORD".to_string(),
                                    value: "123456".to_string()
                                }
                            ],
                            restart: Policy {
                                condition: "always".to_string()
                            },
                            command: Command {
                                value: "mysqld".to_string()
                            },
                            labels: vec![
                                Label {
                                    key: "com.example.description".to_string(),
                                    value: "MySQL Server".to_string()
                                }
                            ], 
                            depends_on: vec![]
                        },
                        Service {
                            image: "redis".to_string(),
                            container_name: "redis".to_string(),
                            ports: vec![
                                Port {
                                    host: "6379".to_string(),
                                    container: "6379".to_string()
                                }
                            ],
                            volumes: vec![
                                Volume {
                                    title: "./redis".to_string(),
                                    driver: "/data".to_string()
                                }
                            ],
                            networks: vec![
                                Network {
                                    driver: "web".to_string(),
                                    external: false
                                }
                            ],
                            environments: vec![
                                Environment {
                                    title: "REDIS_PASSWORD".to_string(),
                                    value: "123456".to_string()
                                }
                            ],
                            restart: Policy {
                                condition: "always".to_string()
                            },
                            command: Command {
                                value: "redis-server".to_string()
                            },
                            labels: vec![
                                Label {
                                    key: "com.example.description".to_string(),
                                    value: "Redis Server".to_string()
                                }
                            ],
                            depends_on: vec![
                                Dependency {
                                    service: "mysql".to_string()
                                }
                            ]
                        },
                        Service {
                            image: "wordpress".to_string(),
                            container_name: "wordpress".to_string(),
                            ports: vec![
                                Port {
                                    host: "8080".to_string(),
                                    container: "80".to_string()
                                }
                            ],
                            volumes: vec![
                                Volume {
                                    title: "./wordpress".to_string(),
                                    driver: "/var/www/html".to_string()
                                }
                            ],
                            networks: vec![
                                Network {
                                    driver: "web".to_string(),
                                    external: false
                                }
                            ],
                            environments: vec![
                                Environment {
                                    title: "WORDPRESS_DB_HOST".to_string(),
                                    value: "mysql:3306".to_string()
                                },
                                Environment {
                                    title: "WORDPRESS_DB_USER".to_string(),
                                    value: "root".to_string()
                                },
                                Environment {
                                    title: "WORDPRESS_DB_PASSWORD".to_string(),
                                    value: "123456".to_string()
                                }
                            ],
                            restart: Policy {
                                condition: "always".to_string()
                            },
                            command: Command {
                                value: "apache2-foreground".to_string()
                            },
                            labels: vec![
                                Label {
                                    key: "com.example.description".to_string(),
                                    value: "WordPress Server".to_string()
                                }
                            ],
                            depends_on: vec![
                                Dependency {
                                    service: "mysql".to_string()
                                }
                            ]
                        },
                        Service {
                            image: "phpmyadmin".to_string(),
                            container_name: "phpmyadmin".to_string(),
                            ports: vec![
                                Port {
                                    host: "8081".to_string(),
                                    container: "80".to_string()
                                }
                            ],
                            volumes: vec![],
                            networks: vec![
                                Network {
                                    driver: "web".to_string(),
                                    external: false
                                }
                            ],
                            environments: vec![
                                Environment {
                                    title: "PMA_HOST".to_string(),
                                    value: "mysql".to_string()
                                },
                                Environment {
                                    title: "PMA_USER".to_string(),
                                    value: "root".to_string()
                                },
                                Environment {
                                    title: "PMA_PASSWORD".to_string(),
                                    value: "123456".to_string()
                                }
                            ],
                            restart: Policy {
                                condition: "always".to_string()
                            },
                            command: Command {
                                value: "apache2-foreground".to_string()
                            },
                            labels: vec![
                                Label {
                                    key: "com.example.description".to_string(),
                                    value: "PhpMyAdmin Server".to_string()
                                }
                            ],
                            depends_on: vec![
                                Dependency {
                                    service: "mysql".to_string()
                                }
                            ]
                        },
                        Service {
                            image: "portainer".to_string(),
                            container_name: "portainer".to_string(),
                            ports: vec![
                                Port {
                                    host: "9000".to_string(),
                                    container: "9000".to_string()
                                }
                            ],
                            volumes: vec![
                                Volume {
                                    title: "/var/run/docker.sock".to_string(),
                                    driver: "/var/run/docker.sock".to_string()
                                }
                            ],
                            networks: vec![
                                Network {
                                    driver: "web".to_string(),
                                    external: false
                                }
                            ],
                            environments: vec![],
                            restart: Policy {
                                condition: "always".to_string()
                            },
                            command: Command {
                                value: "".to_string()
                            },
                            labels: vec![
                                Label {
                                    key: "com.example.description".to_string(),
                                    value: "Portainer Server".to_string()
                                }
                            ],
                            depends_on: vec![]
                        },
                        Service {
                            image: "cadvisor".to_string(),
                            container_name: "cadvisor".to_string(),
                            ports: vec![
                                Port {
                                    host: "8082".to_string(),
                                    container: "8080".to_string()
                                }
                            ],
                            volumes: vec![
                                Volume {
                                    title: "/".to_string(),
                                    driver: "/".to_string()
                                }
                            ],
                            networks: vec![
                                Network {
                                    driver: "web".to_string(),
                                    external: false
                                }
                            ],
                            environments: vec![],
                            restart: Policy {
                                condition: "always".to_string()
                            },
                            command: Command {
                                value: "".to_string()
                            },
                            labels: vec![
                                Label {
                                    key: "com.example.description".to_string(),
                                    value: "Cadvisor Server".to_string()
                                }
                            ],
                            depends_on: vec![]
                        },
                        Service {
                            image: "certbot".to_string(),
                            container_name: "certbot".to_string(),
                            ports: vec![],
                            volumes: vec![
                                Volume {
                                    title: "/etc/letsencrypt".to_string(),
                                    driver: "/etc/letsencrypt".to_string()
                                },
                                Volume {
                                    title: "/var/lib/letsencrypt".to_string(),
                                    driver: "/var/lib/letsencrypt".to_string()
                                },
                                Volume {
                                    title: "/var/log/letsencrypt".to_string(),
                                    driver: "/var/log/letsencrypt".to_string()
                                }
                            ],
                            networks: vec![
                                Network {
                                    driver: "web".to_string(),
                                    external: false
                                }
                            ],
                            environments: vec![],
                            restart: Policy {
                                condition: "always".to_string()
                            },
                            command: Command {
                                value: "".to_string()
                            },
                            labels: vec![
                                Label {
                                    key: "com.example.description".to_string(),
                                    value: "Certbot Server".to_string()
                                }
                            ],
                            depends_on: vec![]
                        },
                        Service {
                            image: "nginx".to_string(),
                            container_name: "nginx".to_string(),
                            ports: vec![
                                Port {
                                    host: "80".to_string(),
                                    container: "80".to_string()
                                },
                                Port {
                                    host: "443".to_string(),
                                    container: "443".to_string()
                                }
                            ],
                            volumes: vec![
                                Volume {
                                    title: "nginx".to_string(),
                                    driver: "/etc/nginx".to_string()
                                }
                            ],
                            networks: vec![
                                Network {
                                    driver: "web".to_string(),
                                    external: false
                                }
                            ],
                            environments: vec![],
                            restart: Policy {
                                condition: "always".to_string()
                            },
                            command: Command {
                                value: "".to_string()
                            },
                            labels: vec![
                                Label {
                                    key: "com.example.description".to_string(),
                                    value: "Nginx Server".to_string()
                                }
                            ],
                            depends_on: vec![
                                Dependency {
                                    service: "wordpress".to_string()
                                },
                                Dependency {
                                    service: "phpmyadmin".to_string()
                                },
                                Dependency {
                                    service: "portainer".to_string()
                                },
                                Dependency {
                                    service: "cadvisor".to_string()
                                },
                                Dependency {
                                    service: "certbot".to_string()
                                }
                            ]
                        },
                        Service {
                            image: "gitlab".to_string(),
                            container_name: "gitlab".to_string(),
                            ports: vec![
                                Port {
                                    host: "8083".to_string(),
                                    container: "80".to_string()
                                },
                                Port {
                                    host: "2222".to_string(),
                                    container: "22".to_string()
                                }
                            ],
                            volumes: vec![
                                Volume {
                                    title: "./gitlab/config".to_string(),
                                    driver: "/etc/gitlab".to_string()
                                },
                                Volume {
                                    title: "./gitlab/logs".to_string(),
                                    driver: "/var/log/gitlab".to_string()
                                },
                                Volume {
                                    title: "./gitlab/data".to_string(),
                                    driver: "/var/opt/gitlab".to_string()
                                }
                            ],
                            networks: vec![
                                Network {
                                    driver: "web".to_string(),
                                    external: false
                                }
                            ],
                            environments: vec![
                                Environment {
                                    title: "GITLAB_OMNIBUS_CONFIG".to_string(),
                                    value: "external_url 'https://gitlab.example.com'; gitlab_rails['gitlab_shell_ssh_port'] = 2222;".to_string()
                                }
                            ],
                            restart: Policy {
                                condition: "always".to_string()
                            },
                            command: Command {
                                value: "".to_string()
                            },
                            labels: vec![
                                Label {
                                    key: "com.example.description".to_string(),
                                    value: "Gitlab Server".to_string()
                                }
                            ],
                            depends_on: vec![]
                        }, 
                        Service {
                            image: "sonarqube".to_string(),
                            container_name: "sonarqube".to_string(),
                            ports: vec![
                                Port {
                                    host: "8084".to_string(),
                                    container: "9000".to_string()
                                }
                            ],
                            volumes: vec![
                                Volume {
                                    title: "./sonarqube/conf".to_string(),
                                    driver: "/opt/sonarqube/conf".to_string()
                                },
                                Volume {
                                    title: "./sonarqube/data".to_string(),
                                    driver: "/opt/sonarqube/data".to_string()
                                },
                                Volume {
                                    title: "./sonarqube/logs".to_string(),
                                    driver: "/opt/sonarqube/logs".to_string()
                                },
                                Volume {
                                    title: "./sonarqube/extensions".to_string(),
                                    driver: "/opt/sonarqube/extensions".to_string()
                                }
                            ],
                            networks: vec![
                                Network {
                                    driver: "web".to_string(),
                                    external: false
                                }
                            ],
                            environments: vec![],
                            restart: Policy {
                                condition: "always".to_string()
                            },
                            command: Command {
                                value: "".to_string()
                            },
                            labels: vec![
                                Label {
                                    key: "com.example.description".to_string(),
                                    value: "Sonarqube Server".to_string()
                                }
                            ],
                            depends_on: vec![]
                        }, 
                        Service {
                            image: "code".to_string(),
                            container_name: "code-server".to_string(),
                            ports: vec![
                                Port {
                                    host: "8085".to_string(),
                                    container: "8080".to_string()
                                }
                            ],
                            volumes: vec![
                                Volume {
                                    title: "./code-server".to_string(),
                                    driver: "/home/coder/project".to_string()
                                }
                            ],
                            networks: vec![
                                Network {
                                    driver: "web".to_string(),
                                    external: false
                                }
                            ],
                            environments: vec![
                                Environment {
                                    title: "PASSWORD".to_string(),
                                    value: "123456".to_string()
                                }
                            ],
                            restart: Policy {
                                condition: "always".to_string()
                            },
                            command: Command {
                                value: "".to_string()
                            },
                            labels: vec![
                                Label {
                                    key: "com.example.description".to_string(),
                                    value: "Code Server".to_string()
                                }
                            ],
                            depends_on: vec![]
                        },
                        Service {
                            image: "mailhog/mailhog".to_string(),
                            container_name: "mailhog".to_string(),
                            ports: vec![
                                Port {
                                    host: "8086".to_string(),
                                    container: "8025".to_string()
                                }
                            ],
                            volumes: vec![],
                            networks: vec![
                                Network {
                                    driver: "web".to_string(),
                                    external: false
                                }
                            ],
                            environments: vec![],
                            restart: Policy {
                                condition: "always".to_string()
                            },
                            command: Command {
                                value: "".to_string()
                            },
                            labels: vec![
                                Label {
                                    key: "com.example.description".to_string(),
                                    value: "Mailhog Server".to_string()
                                }
                            ],
                            depends_on: vec![]
                        }, 
                        Service {
                            image: "pmsipilot/docker-compose-viz".to_string(),
                            container_name: "graphviz".to_string(),
                            ports: vec![
                                Port {
                                    host: "8087".to_string(),
                                    container: "8080".to_string()
                                }
                            ],
                            volumes: vec![
                                Volume {
                                    title: "/var/run/docker.sock".to_string(),
                                    driver: "/var/run/docker.sock".to_string()
                                }
                            ],
                            networks: vec![
                                Network {
                                    driver: "web".to_string(),
                                    external: false
                                }
                            ],
                            environments: vec![],
                            restart: Policy {
                                condition: "always".to_string()
                            },
                            command: Command {
                                value: "".to_string()
                            },
                            labels: vec![
                                Label {
                                    key: "com.example.description".to_string(),
                                    value: "Graphviz Server".to_string()
                                }
                            ],
                            depends_on: vec![]
                        }
                    ],
                    volumes: vec![
                        Volume {
                            title: "nginx".to_string(),
                            driver: "local".to_string()
                        },
                        Volume {
                            title: "mysql".to_string(),
                            driver: "local".to_string()
                        },
                        Volume {
                            title: "redis".to_string(),
                            driver: "local".to_string()
                        },
                        Volume {
                            title: "wordpress".to_string(),
                            driver: "local".to_string()
                        },
                        Volume {
                            title: "phpmyadmin".to_string(),
                            driver: "local".to_string()
                        },
                        Volume {
                            title: "portainer".to_string(),
                            driver: "local".to_string()
                        },
                        Volume {
                            title: "cadvisor".to_string(),
                            driver: "local".to_string()
                        },
                        Volume {
                            title: "certbot".to_string(),
                            driver: "local".to_string()
                        },
                        Volume {
                            title: "gitlab".to_string(),
                            driver: "local".to_string()
                        },
                        Volume {
                            title: "sonarqube".to_string(),
                            driver: "local".to_string()
                        },
                        Volume {
                            title: "code-server".to_string(),
                            driver: "local".to_string()
                        },
                        Volume {
                            title: "mailhog".to_string(),
                            driver: "local".to_string()
                        },
                        Volume {
                            title: "graphviz".to_string(),
                            driver: "local".to_string()
                        }
                    ],
                    networks: vec![
                        Network {
                            driver: "web".to_string(),
                            external: false
                        },
                        Network {
                            driver: "hub".to_string(),
                            external: false
                        }
                    ],
                    environments: vec![
                        Environment {
                            title: "VIRTUAL_HOST".to_string(),
                            value: "example.com".to_string()
                        },
                        Environment {
                            title: "LETSENCRYPT_HOST".to_string(),
                            value: "example.com".to_string()
                        },
                        Environment {
                            title: "LETSENCRYPT_EMAIL".to_string(),
                            value: "ydev@tuta.io".to_string()
                        }
                    ],
                    extensions: vec![
                        Extension {
                            title: "traefik.enable".to_string(),
                            value: "example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.rule".to_string(),
                            value: "HostRegexp(`{any:.+}`)".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.entrypoints".to_string(),
                            value: "web".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.middlewares".to_string(),
                            value: "redirect-to-https".to_string()
                        },
                        Extension {
                            title: "traefik.http.middlewares.redirect-to-https.redirectscheme.scheme".to_string(),
                            value: "https".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.priority".to_string(),
                            value: "1".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls".to_string(),
                            value: "true".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.certresolver".to_string(),
                            value: "letsencrypt".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].main".to_string(),
                            value: "example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].sans[0]".to_string(),
                            value: "www.example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].sans[1]".to_string(),
                            value: "gitlab.example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].sans[2]".to_string(),
                            value: "sonarqube.example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].sans[3]".to_string(),
                            value: "code-server.example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].sans[4]".to_string(),
                            value: "mailhog.example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].sans[5]".to_string(),
                            value: "graphviz.example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].sans[6]".to_string(),
                            value: "portainer.example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].sans[7]".to_string(),
                            value: "cadvisor.example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].sans[8]".to_string(),
                            value: "phpmyadmin.example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].sans[9]".to_string(),
                            value: "wordpress.example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].sans[10]".to_string(),
                            value: "redis.example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].sans[11]".to_string(),
                            value: "mysql.example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].sans[12]".to_string(),
                            value: "nginx.example.com".to_string()
                        },
                        Extension {
                            title: "traefik.http.routers.http-catchall.tls.domains[0].sans[13]".to_string(),
                            value: "hub.example.com".to_string()
                        },
                        Extension {
                            title: "nginx.http.routers.nginx.rule".to_string(),
                            value: "Host(`nginx.example.com`)".to_string()
                        },
                        Extension {
                            title: "nginx.http.routers.nginx.entrypoints".to_string(),
                            value: "web".to_string()
                        },
                        Extension {
                            title: "nginx.http.routers.nginx.tls".to_string(),
                            value: "true".to_string()
                        },
                        Extension {
                            title: "nginx.http.routers.nginx.tls.certresolver".to_string(),
                            value: "letsencrypt".to_string()
                        },
                        Extension {
                            title: "nginx.http.routers.nginx.tls.domains[0].main".to_string(),
                            value: "nginx.example.com".to_string()
                        },
                        Extension {
                            title: "nginx.http.routers.nginx.tls.domains[0].sans[0]".to_string(),
                            value: "www.nginx.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.rule".to_string(),
                            value: "Host(`hub.example.com`)".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.entrypoints".to_string(),
                            value: "web".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls".to_string(),
                            value: "true".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.certresolver".to_string(),
                            value: "letsencrypt".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].main".to_string(),
                            value: "hub.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].sans[0]".to_string(),
                            value: "www.hub.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].sans[1]".to_string(),
                            value: "gitlab.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].sans[2]".to_string(),
                            value: "sonarqube.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].sans[3]".to_string(),
                            value: "code-server.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].sans[4]".to_string(),
                            value: "mailhog.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].sans[5]".to_string(),
                            value: "graphviz.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].sans[6]".to_string(),
                            value: "portainer.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].sans[7]".to_string(),
                            value: "cadvisor.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].sans[8]".to_string(),
                            value: "phpmyadmin.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].sans[9]".to_string(),
                            value: "wordpress.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].sans[10]".to_string(),
                            value: "redis.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].sans[11]".to_string(),
                            value: "mysql.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].sans[12]".to_string(),
                            value: "nginx.example.com".to_string()
                        },
                        Extension {
                            title: "hub.http.routers.hub.tls.domains[0].sans[13]".to_string(),
                            value: "hub.example.com".to_string()
                        }
                    ]
                };
                let compose_str = parse_compose(compose);
                actix_web::HttpResponse::Ok().body(compose_str)
             }))
    })
    .bind("0.0.0.0:8888")?
    .run()
    .await
}
