use std::collections::HashMap;

use wasm_bindgen::JsCast;
use web_sys::{Attr, HtmlCollection, NamedNodeMap, NodeList};

pub fn html_collection_to_vec<T: JsCast>(collection: HtmlCollection) -> Vec<T> {
    let mut result = Vec::with_capacity(
        collection
            .length()
            .try_into()
            .expect("usize should be at least u32."),
    );
    for i in 0..collection.length() {
        result.push(
            collection
                .item(i)
                .expect("Item should exist.")
                .unchecked_into::<T>(),
        );
    }
    result
}

pub fn named_node_map_to_vec(named_node_map: NamedNodeMap) -> Vec<Attr> {
    let mut result = Vec::with_capacity(
        named_node_map
            .length()
            .try_into()
            .expect("usize should be at least u32."),
    );
    for i in 0..named_node_map.length() {
        let attr = named_node_map.item(i).expect("Item should exist.");
        result.push(attr);
    }
    result
}

pub fn named_node_map_to_hashmap(named_node_map: NamedNodeMap) -> HashMap<String, String> {
    let mut result = HashMap::with_capacity(
        named_node_map
            .length()
            .try_into()
            .expect("usize should be at least u32."),
    );
    for i in 0..named_node_map.length() {
        let attr = named_node_map.item(i).expect("Item should exist.");
        result.insert(attr.name(), attr.value());
    }
    result
}

pub fn node_list_to_vec<T: JsCast>(node_list: NodeList) -> Vec<T> {
    let mut result = Vec::with_capacity(
        node_list
            .length()
            .try_into()
            .expect("usize should be at least u32."),
    );
    for i in 0..node_list.length() {
        result.push(
            node_list
                .item(i)
                .expect("Item should exist.")
                .unchecked_into::<T>(),
        );
    }
    result
}
