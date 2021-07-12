use crate::proxy::net::bridge::{ip_netns, NetEnv};

pub fn set_iptables<'a>(
    net_env: &'a NetEnv,
    proxy_ports: Option<&'a str>,
    listen_port: &'a str,
) -> Vec<Vec<&'a str>> {
    let cmdv = match proxy_ports {
        Some(proxy_ports) => ip_netns(
            &net_env.netns,
            vec![
                "iptables",
                "-t",
                "mangle",
                "-A",
                "PREROUTING",
                "-p",
                "tcp",
                "-m",
                "multiport",
                "--dports",
                proxy_ports,
                "-j",
                "TPROXY",
                "--tproxy-mark",
                "0x1/0x1",
                "--on-port",
                listen_port,
            ],
        ),
        None => ip_netns(
            &net_env.netns,
            vec![
                "iptables",
                "-t",
                "mangle",
                "-A",
                "PREROUTING",
                "-p",
                "tcp",
                "-j",
                "TPROXY",
                "--tproxy-mark",
                "0x1/0x1",
                "--on-port",
                listen_port,
            ],
        ),
    };
    vec![
        ip_netns(
            &net_env.netns,
            vec!["iptables", "-t", "mangle", "-N", "DIVERT"],
        ),
        ip_netns(
            &net_env.netns,
            vec![
                "iptables",
                "-t",
                "mangle",
                "-A",
                "PREROUTING",
                "-p",
                "tcp",
                "-m",
                "socket",
                "-j",
                "DIVERT",
            ],
        ),
        ip_netns(
            &net_env.netns,
            vec![
                "iptables",
                "-t",
                "mangle",
                "-A",
                "DIVERT",
                "-j",
                "MARK",
                "--set-mark",
                "1",
            ],
        ),
        ip_netns(
            &net_env.netns,
            vec!["iptables", "-t", "mangle", "-A", "DIVERT", "-j", "ACCEPT"],
        ),
        cmdv,
        ip_netns(
            &net_env.netns,
            vec![
                "ebtables-legacy",
                "-t",
                "broute",
                "-A",
                "BROUTING",
                "-p",
                "IPv4",
                "--ip-proto",
                "6",
                "-j",
                "redirect",
                "--redirect-target",
                "DROP",
            ],
        ),
    ]
}

pub fn set_iptables_safe(net_env: &NetEnv) -> Vec<Vec<&str>> {
    vec![
        ip_netns(
            &net_env.netns,
            vec![
                "iptables",
                "-t",
                "mangle",
                "-I",
                "PREROUTING",
                "-p",
                "tcp",
                "--dport",
                "81:1025",
                "-s",
                &net_env.ip,
                "-j",
                "ACCEPT",
            ],
        ),
        ip_netns(
            &net_env.netns,
            vec![
                "iptables",
                "-t",
                "mangle",
                "-I",
                "PREROUTING",
                "-p",
                "tcp",
                "--sport",
                "81:1025",
                "-d",
                &net_env.ip,
                "-j",
                "ACCEPT",
            ],
        ),
        ip_netns(
            &net_env.netns,
            vec![
                "iptables",
                "-t",
                "mangle",
                "-I",
                "PREROUTING",
                "-p",
                "tcp",
                "--dport",
                "1:81",
                "-s",
                &net_env.ip,
                "-j",
                "ACCEPT",
            ],
        ),
        ip_netns(
            &net_env.netns,
            vec![
                "iptables",
                "-t",
                "mangle",
                "-I",
                "PREROUTING",
                "-p",
                "tcp",
                "--sport",
                "1:81",
                "-d",
                &net_env.ip,
                "-j",
                "ACCEPT",
            ],
        ),
    ]
}
