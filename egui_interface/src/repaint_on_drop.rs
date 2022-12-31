pub struct Repainter {
    ctx: egui::Context,
}

impl Drop for Repainter {
    fn drop(&mut self) {
        self.ctx.request_repaint();
    }
}

pub trait Repaint {
    #[must_use]
    fn repaint_on_drop(&self) -> Repainter;
}

impl Repaint for egui::Context {
    fn repaint_on_drop(&self) -> Repainter {
        Repainter { ctx: self.clone() }
    }
}
