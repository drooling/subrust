use std::fs;
use std::io::Write;
use std::time::Duration;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

fn subdomain_finder(base_domain: &str) {
    let subdomains = fs::read_to_string("./subdomains.txt").unwrap();
    
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(1);
    opts.ip_strategy = LookupIpStrategy::Ipv4Only;

    let resolver = Resolver::new(ResolverConfig::default(), opts).unwrap();
    for subdomain in subdomains.lines() {
        let records = match resolver.lookup_ip(format!("{}.{}", subdomain.trim(), base_domain)) {
            Ok(rec) => rec,
            Err(_) => continue,
        };
        for record in records {
            println!("[ + ] {}.{} -> {}", subdomain.trim(), base_domain, &record)
        }
    }
}

fn main() {
    let mut input = String::new();
    print!("Domain: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("ion know how to read");
    let domain = input.trim().to_string();
    subdomain_finder(domain.as_str());
}