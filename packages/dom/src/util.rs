use wasm_bindgen::JsCast;
use web_sys::NodeList;

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
                .get(i)
                .expect("Node should exist.")
                .unchecked_into::<T>(),
        );
    }
    result
}
