use crate::app::traits::Processor;

pub trait Listener {
    type K;

    fn listen<T: Processor>(&mut self, obj: &mut T) -> Result<(), Self::K>;
}
