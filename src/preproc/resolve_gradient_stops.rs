// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use svgdom::{
    Document,
    ElementType,
    Node,
};

use short::{
    AId,
    AValue,
};


pub fn resolve_gradient_stops(doc: &Document) {
    let iter = doc.descendants().filter(|n| n.is_gradient())
                                .filter(|n| n.has_attribute(AId::XlinkHref))
                                .filter(|n| !n.has_children());
    for mut node in iter {
        let link = node.clone();
        resolve(&mut node, &link);
    }

    // Remove 'xlink:href' in gradients, because we already resolved everything.
    let iter = doc.descendants().filter(|n| n.is_gradient())
                                .filter(|n| n.has_attribute(AId::XlinkHref));
    for mut node in iter {
        node.remove_attribute(AId::XlinkHref);
    }
}

fn resolve(gradient: &mut Node, linked_gradient: &Node) {
    let av = linked_gradient.attributes().get_value(AId::XlinkHref).cloned();
    match av {
        Some(av) => {
            match av {
                AValue::Link(ref_node) => resolve(gradient, &ref_node),
                _ => unreachable!(),
            }
        }
        None => {
            for stop in linked_gradient.children() {
                let new_stop = stop.make_copy();
                gradient.append(&new_stop);
            }
        }
    }
}
