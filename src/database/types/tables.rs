macro_rules! table {
    (name: $name:ident, fields: { $( $field:ident : $typ:ty $(,)? )* }) => {
        paste::paste!{
            pub mod [<table_ $name>] {
                use crate::database::types::*;
                use crate::utils::AsRes;
                use crate::handler::Res;
                use hyper::body::Bytes;
                use http_body_util::Full;

                pub static TABLE_NAME: &str = stringify!($name);

                $(
                    pub static [< $field:upper >] : &str = stringify!($field);
                )*

                #[derive(Serialize, Deserialize)]
                #[serde(rename_all="camelCase")]
                pub struct Object {
                    pub id: ID,
                    $(
                        pub $field: $typ,
                    )*
                }

                #[derive(Serialize, Deserialize, Default)]
                #[serde(rename_all="camelCase")]
                pub struct ObjectAny {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub id: Option<ID>,
                    $(
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub $field: Option<$typ>,
                    )*
                }

                #[derive(Deserialize, Copy, Clone, Default)]
                #[serde(rename_all="camelCase")]
                pub struct FieldGetter {
                    pub id: Option<bool>,
                    $(
                        pub $field: Option<bool>,
                    )*
                }

                impl AsRes for Object {
                    fn into_res(mut self) -> Res {
                        self.id = self.id.map_to_raw();
                        Res::new(Full::new(Bytes::from(
                            serde_json::to_string(&self).unwrap()
                        )))
                    }
                }

                impl AsRes for ObjectAny {
                    fn into_res(mut self) -> Res {
                        self.id = self.id.take().map(ID::map_to_raw);
                        Res::new(Full::new(Bytes::from(
                            serde_json::to_string(&self).unwrap()
                        )))
                    }
                }

                impl AsRes for Vec<ObjectAny> {
                    fn into_res(mut self) -> Res where Self: Sized {
                        self = self
                            .into_iter()
                            .map(|mut obj| {
                                obj.id = obj.id.map(ID::map_to_raw);
                                obj
                            })
                            .collect::<Vec<ObjectAny>>();

                        Res::new(Full::new(Bytes::from(
                            serde_json::to_string(&self).unwrap()
                        )))
                    }
                }

                impl ObjectAny {
                    pub fn only_selected(&mut self, fields: FieldGetter) {
                        if !fields.id.unwrap_or_default() {
                            let _ = self.id.take();
                        }

                        $(
                            if !fields.$field.unwrap_or_default() {
                                let _ = self.$field.take();
                            }
                        )*
                    }
                }
            }
        }
    };
}

/* ===== [ EXAMPLE ] =====
    table! {
        name: user,
        fields: {
            name: String,
            email: String,
            birth: Option<u64>,
            picture: Option<String>,
        }
    }
*/
