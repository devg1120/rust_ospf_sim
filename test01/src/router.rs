use std::collections::LinkedList;
use std::net::IpAddr;

#[derive(Debug)]
pub struct Route {
     pub dest: String,
     pub metric: i32,
}

#[derive(Debug)]
pub struct RouteTable {
    pub route: LinkedList<Route>,
}

impl RouteTable {
    pub fn new() -> RouteTable{
        RouteTable {
           route:  {LinkedList::new()}
        }
        
    }
    pub fn add_route(&mut self, route: Route) {
       self.route.push_back(route);
    }


}

#[derive(Debug)]
pub struct Iface<'a> {
     pub name: String,
     pub bandwith: i32,
     pub address: IpAddr,
     pub netmask: i32,
     pub router: &'a Router<'a>,
}

#[derive(Debug)]
pub struct Router<'a> {

    pub hostname: String,
    pub iface: LinkedList<Iface<'a>>,
    pub routetable: RouteTable,
}

impl<'a> Router<'a> {

    pub fn set_hostname(&mut self, name:String) {
       self.hostname = name;
    }

    pub fn get_hostname(&self )-> &String {
       &self.hostname 
    }

    pub fn add_route(&mut self, route: Route) {
       self.routetable.add_route(route);
    }

    pub fn add_iface(&mut self, iface: Iface<'static>) {
       self.iface.push_back(iface);
    }

}

