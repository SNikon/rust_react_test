use js_sys::Reflect;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_react::{ c, Component, export_components, h, VNode };


struct Sample {
    count: Option<f64>,
    description: Option<Rc<str>>,
    label: Option<Rc<str>>
}

impl Component for Sample {
    fn render(&self) -> VNode {
      h!(div)
        .build(c![
          h!(h).build(c![
            "Counter: ",
            if let Some(label) = self.label.as_ref() { label } else { "" }
        ]),
          h!(button).build(c!["Increment"]),
        ])
    }
}

impl TryFrom<JsValue> for Sample {
    type Error = JsValue;
  
    fn try_from(props: JsValue) -> Result<Self, Self::Error> {
      Ok(Sample {
        count: Reflect::get(&props, &"count".into())?
            .as_f64(),
        description: Reflect::get(&props, &"description".into())?
            .as_string()
            .map(|x| x.into_boxed_str().into()),
        label: Reflect::get(&props, &"label".into())?
            .as_string()
            .map(|x| x.into_boxed_str().into()),
        
      })
    }
  }

export_components! { Sample }

#[wasm_bindgen]
pub fn render(target_id: &str) {
    let dom_document = web_sys::window().unwrap().document().unwrap();
    let target_element = dom_document.get_element_by_id(target_id).unwrap();
    target_element.set_inner_html("Banana");
}
