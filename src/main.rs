////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
//
// $; i3-switcher.rs
//
// @; description: 
//    it's little hacky for the i3wm that providies back-and-forth behavior 
//    between current and last focused window.
//                 
// @; dependencies: 
//    * i3ipc-rs
//    * regex
//
// @; author: galvares <galvares@afu.blue>
// @; version: 0.1.0
// @; dated: 2017-12-14
//
////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
//
// @; OBS
//
// * Controller::node_iterator():
//
// The logic used to implement the iterator into i3ipc::reply::Node was:
//
// 1. if parent Node has childrens, iter into childrens;
// 2. if children Node has childrens, iter into childrens;
// 3. if Node is window and Node is focused, break iteration and..
//    ..return window ID;
// 4. or if results of iteration is focused > 0, return focused;
//
// The Rust-way:
// 
// I know that the best way to implement this is implementing the..
// ..IntoIterator and Iter for specific struct. (But i3ipc::reply::Node..
// ..is part of i3ipc-rs crate, and I couldn't implement directly into it.)
//
// The method described above is something like this:
//
// struct Node {
//     id: i64,
//     nodes: Vec<Node>,
// }
//
// impl<'a> IntoIterator for &'a Node {
//     type Item = &'a Node;
//     type IntoIter = Iter<'a>;
//     fn into_iter(self) -> Self::IntoIter {
//         Iter(vec![self])
//     }
// }
//
// struct Iter<'a>(Vec<&'a Node>);
//
// impl<'a> Iterator for Iter<'a> {
//     type Item = &'a Node;
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.0.pop() {
//             Some(n) => {
//                 self.0.extend(&n.nodes);
//                 Some(n)
//             }
//             None => None,
//         }
//     }
// }
//
// fn main() {
//     let n = Node { id: 1, nodes: vec![
//         Node { id: 2, nodes: vec![ 
//             Node { id: 3, nodes: vec![] }, 
//             Node { id: 4, nodes: vec![] },
//             Node { id: 5, nodes: vec![
//                 Node { id: 6, nodes: vec![]},
//                 Node { id: 7, nodes: vec![
//                     Node { id: 8, nodes: vec![] },
//                     Node { id: 9, nodes: vec![] },
//                 ] }
//             ] }
//         ] }
//     ] };
//
//     for nn in &n {
//         println!("{}", nn.id);
//     }
// }
//
////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

// regex
extern crate regex;
use regex::Regex;

// i3ipc
extern crate i3ipc;
use i3ipc::I3Connection;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::event::Event;
use i3ipc::event::inner::{
    WindowChange,
    BindingChange
};

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

// Controller struct and implementation
pub struct Controller
{
    current: u32,
    latest: u32,
    connection: I3Connection,
}

impl Controller
{
    fn new() -> Controller
    {
        Controller
        {
            current: 0,
            latest: 0,
            connection: I3Connection::connect().unwrap()            
        }
    }
    
    pub fn __prepare_focused(&mut self)
    {
        self.set_focused();

        self.latest  = self.current;
        self.current = self.current;
    }
    
    pub fn set_focused(&mut self)
    {
        let focused: u32 = self.get_focused();

        if focused > 0
        {
            if focused != self.current && focused != self.latest
            {
                self.latest  = self.current;
                self.current = focused;
            }
            else if focused == self.latest
            {
                let latest   = self.latest;
                
                self.latest  = self.current;
                self.current = latest;
            }
        }
    }

    pub fn get_focused(&mut self) -> u32
    {
        let mut focused: u32 = 0;

        for node in self.connection.get_tree().iter()
        {
            focused = self.node_iterator(node);
        }
        focused
    }

    pub fn back_and_forth(&mut self)
    {
        let command = format!("[id={}] focus", self.latest);               
        self.connection.run_command(&*command).unwrap();
    }

    #[allow(unused_assignments)]
    pub fn node_iterator(&self, node: &i3ipc::reply::Node) -> u32
    {        
        let mut focused: u32 = 0;

        if node.nodes.len() > 0
        {
            for n in node.nodes.iter()
            {
                if n.nodes.len() > 0
                {
                    focused = self.node_iterator(n);

                    if focused > 0
                    {
                        return focused;
                    }
                }                
                else
                {
                    if n.window != None && n.focused == true
                    {
                        return n.window.unwrap() as u32;
                    }
                }                
            }
        }
        return 0;        
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

// Listener struct and implementation
pub struct Listener
{
    listener: I3EventListener
}

impl Listener
{
    fn new() -> Listener
    {
        Listener
        {
            listener: I3EventListener::connect().unwrap()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

// main
#[allow(unused_variables)]
fn main()
{
    let mut ctrl = Controller::new();    
    let mut event = Listener::new();   
    
    ctrl.__prepare_focused();

    event.listener.subscribe(&[
        Subscription::Binding,
        Subscription::Window
    ]).unwrap();    

    for e in event.listener.listen()
    {
        match e.unwrap()
        {
            Event::BindingEvent(v) =>
            {
                match v.change
                {
                    BindingChange::Run => {
                        if Regex::new(r"^exec true$")
                            .unwrap()
                            .is_match(&*v.binding.command)
                        {
                            ctrl.back_and_forth();
                        }
                    },
                    _ => {}
                }
            },
            Event::WindowEvent(v) =>
            {
                match v.change
                {
                    WindowChange::Focus => ctrl.set_focused(),
                    _ => {}
                }                
            },
            _ => unreachable!()
        }
    }
}
