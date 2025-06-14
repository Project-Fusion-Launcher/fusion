use crate::components::Modal;
use gpui::*;
use std::rc::Rc;

pub trait PortalContext: Sized {
    fn open_modal<F>(&mut self, cx: &mut App, build: F)
    where
        F: Fn(Modal, &mut Window, &mut App) -> Modal + 'static;

    fn close_modal(&mut self, cx: &mut App);
}

impl PortalContext for Window {
    fn open_modal<F>(&mut self, cx: &mut App, build: F)
    where
        F: Fn(Modal, &mut Window, &mut App) -> Modal + 'static,
    {
        ComponentContextProvider::update(self, cx, move |root, window, cx| {
            // Only save focus handle if there are no active modals.
            // This is used to restore focus when all modals are closed.
            if root.active_modals.is_empty() {
                root.previous_focus_handle = window.focused(cx);
            }

            let focus_handle = cx.focus_handle();
            focus_handle.focus(window);

            root.active_modals.push(ActiveModal {
                focus_handle,
                builder: Rc::new(build),
            });
            cx.notify();
        })
    }

    fn close_modal(&mut self, cx: &mut App) {
        ComponentContextProvider::update(self, cx, move |root, window, cx| {
            root.active_modals.pop();

            if let Some(top_modal) = root.active_modals.last() {
                // Focus the next modal.
                top_modal.focus_handle.focus(window);
            } else {
                // Restore focus if there are no more modals.
                root.focus_back(window, cx);
            }
            cx.notify();
        })
    }
}

pub struct ComponentContextProvider {
    previous_focus_handle: Option<FocusHandle>,
    pub(crate) active_modals: Vec<ActiveModal>,
    view: AnyView,
}

#[allow(clippy::type_complexity)]
#[derive(Clone)]
pub(crate) struct ActiveModal {
    focus_handle: FocusHandle,
    builder: Rc<dyn Fn(Modal, &mut Window, &mut App) -> Modal + 'static>,
}

impl ComponentContextProvider {
    pub fn new(view: AnyView, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            previous_focus_handle: None,
            active_modals: Vec::new(),
            view,
        }
    }

    pub fn update<F>(window: &mut Window, cx: &mut App, f: F)
    where
        F: FnOnce(&mut Self, &mut Window, &mut Context<Self>) + 'static,
    {
        if let Some(Some(provider)) = window.root::<Self>() {
            provider.update(cx, |provider, cx| f(provider, window, cx));
        }
    }

    pub fn read<'a>(window: &'a Window, cx: &'a App) -> &'a Self {
        window
            .root::<Self>()
            .expect("The window root view should be of type `ui::ComponentContextProvider`.")
            .unwrap()
            .read(cx)
    }

    fn focus_back(&mut self, window: &mut Window, _: &mut App) {
        if let Some(handle) = self.previous_focus_handle.clone() {
            window.focus(&handle);
        }
    }

    pub fn render_modals(window: &mut Window, cx: &mut App) -> Option<impl IntoElement> {
        let root = window.root::<Self>()??;

        let active_modals = root.read(cx).active_modals.clone();

        if active_modals.is_empty() {
            return None;
        }

        let mut show_overlay_ix = None;

        let mut modals = active_modals
            .iter()
            .enumerate()
            .map(|(i, active_modal)| {
                let mut modal = Modal::new(window, cx);

                modal = (active_modal.builder)(modal, window, cx);

                // Give the modal the focus handle, because `modal` is a temporary value, is not possible to
                // keep the focus handle in the modal.
                //
                // So we keep the focus handle in the `active_modal`, this is owned by the `Root`.
                modal.focus_handle = active_modal.focus_handle.clone();

                modal.layer_ix = i;
                // Find the modal which one needs to show overlay.
                if modal.has_overlay() {
                    show_overlay_ix = Some(i);
                }

                modal
            })
            .collect::<Vec<_>>();

        if let Some(ix) = show_overlay_ix {
            if let Some(modal) = modals.get_mut(ix) {
                modal.overlay_visible = true;
            }
        }

        Some(div().children(modals))
    }

    pub fn view(&self) -> &AnyView {
        &self.view
    }
}

impl Render for ComponentContextProvider {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("root")
            .relative()
            .size_full()
            .child(self.view.clone())
    }
}
