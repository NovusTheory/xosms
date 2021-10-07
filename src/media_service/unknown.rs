// The purpose of this file is to ensure that the module compiles for every platform, but if the platform is not supported then it will noop every method

use neon::prelude::*;

pub struct MediaService {}

impl MediaService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn to_object<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();
        Ok(obj)
    }
}
