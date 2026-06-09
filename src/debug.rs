#[macro_export]
macro_rules! global_panic {
    (asset $type:ident $id:expr) => {
        {
            // This assumes your registry modules are under crate::assets::
            let similar = $crate::assets::$type::find_similar($id);
            let suggestion = if similar.is_empty() {
                "No similar keys found.".to_string()
            } else {
                format!("Did you mean: {}?", similar.join(", "))
            };
            panic!("Registry Error: Could not find {} '{}'.\n{}", stringify!($type), $id, suggestion);
        }
    };
    (data $type:ident $id:expr) => {
        {
            let similar = $crate::assets::$type::find_similar($id);
            let suggestion = if similar.is_empty() {
                "No similar keys found.".to_string()
            } else {
                format!("Did you mean: {}?", similar.join(", "))
            };
            panic!("Registry Error: Could not find {} blueprint '{}'.\n{}", stringify!($type), $id, suggestion);
        }
    };
    (palette $scheme:expr => $palette:expr) => {
        {
            let similar = $palette.find_similar($scheme, 3);
            let suggestion = if similar.is_empty() {
                "No similar keys found.".to_string()
            } else {
                format!("Did you mean: {}?", similar.join(", "))
            };
            panic!("Palette Error: Could not find {} layer in '{}'.\n{}", $scheme, $palette.name, suggestion);
        }
    };
    (empty $type:expr, $object:expr) => {
        {
            panic!("Empty Vector Error: Vector within {} was found empty!\n{:#?}", $type, $object);
        }
    };
    (mismatch $type:expr, $value1:expr => $value2:expr) => {
        {
            panic!("Mismatched Value Error: Values of type {} were not found to be equal!\n{:#?}\n========\n{:#?}", $type, $value1, $value2);
        }
    };
    (unreachable $subject:expr) => {
        {
            panic!("Workflow Error: Intentionally unreachable code regarding {} executed!", $subject);
        }
    };
    (uiaction $action:expr) => {
        panic!("Ui Action Error: Uncovered Ui action:\n{:#?}!", $action)
    };
    (gamestate $data:expr) => {
        panic!("Game State Error: Action Attempted in wrong game state:\n{:#?}!", $data)
    }
}

pub fn debug_frame() -> bool {
    crate::DEBUG.lock().unwrap().clone()
}