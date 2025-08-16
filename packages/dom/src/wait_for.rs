use crate::types::WaitForOptions;

pub async fn wait_for<T>(callback: Box<dyn Fn() -> T>, _options: WaitForOptions) -> T {
    callback()
}
