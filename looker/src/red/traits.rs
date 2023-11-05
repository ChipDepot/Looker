use crate::app::traits::Processor;

trait Listener {
    fn listen<T: Processor, K>(&mut self, obj: &mut T) -> Result<(), K>;
}
