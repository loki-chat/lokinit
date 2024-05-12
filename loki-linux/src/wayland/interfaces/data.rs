use super::surface::Surface;

/// Represents an offer to transfer data coming from another Wayland client.
/// It's used by the client receiving data.
pub struct DataOffer {}
impl DataOffer {
    /// Accept data of the given MIME type from another client.
    ///
    /// # API Differences
    ///
    /// On [`DataOffer`]s v2 or older, this only indicates if the client is
    /// able to receive the specified MIME type. It doesn't affect if the data
    /// ends up being transferred or not.
    ///
    /// On [`DataOffer`]s v3 or newer, this *does* affect if data is transferred
    /// or not.
    pub fn accept(&self, serial_number: u32, mime_type: Option<&str>) {
        todo!()
    }

    /// Request that data from the specified MIME type is sent from the other Wayland
    /// client. That client will write the data to the specified file descriptor.
    pub fn receive(&self, mime_type: &str, file_descriptor: *mut ()) {
        todo!()
    }

    /// Destroy this data offer.
    pub fn destroy(self) {
        todo!()
    }

    /// Inform the other Wayland client that the data from a drag-and-drop it started has finished
    /// being transferred. The Wayland compositor will then send a [`DataOfferListener::dnd_finished`]
    /// event to the other client.
    ///
    /// If the MIME type was set to `None` in [`DataOffer::accept`], or no data was received from
    /// [`DataOfferListener::action`], or the current data offer isn't from a drag-and-drop,
    /// it is an error to call this method.
    pub fn finish(self) {
        todo!()
    }

    /// Sets the available and preferred drag-and-drop actions for this data transfer.
    ///
    /// This method can be called multiple times during a data transfer. If no action is accepted, the
    /// Wayland compositor will send the [`DataOfferListener::cancelled`] event to the other client.
    ///
    /// If the action is set to [`DataOfferDndAction::Ask`], the client can call [`DataOffer::receive`]
    /// to look at the data, but is expected to call this method again later and set the action to something
    /// other than [`DataOfferDndAction::Ask`].
    pub fn set_actions(&self, actions: DataOfferDndActions, preferred_action: DataOfferDndAction) {
        todo!()
    }
}

/// Events from the [`DataOfferListener`] interface.
pub trait DataOfferListener {
    /// This event is sent right after the [`DataOffer`] object is created. It's sent once for every offered
    /// MIME type.
    fn offer(&self, mime_type: &str);

    /// This indicates the actions offered by the Wayland client that started the [`DataOffer`].
    fn source_actions(&self, actions: DataOfferDndActions);

    /// This event fires during a drag-and-drop operation, specifying the action both clients agreed upon.
    /// This event can keep firing until [`DataDevice::drop`] is called, after which the last agreed upon action
    /// or preferred action must be used.
    fn action(&self, action: DataOfferDndAction);
}

#[repr(u32)]
pub enum DataOfferDndAction {
    None = 0,
    Copy = 1,
    Move = 2,
    Ask = 4,
}
/// A version of [`DataOfferDndAction`] that can hold multiple actions. Use [`DataOfferDndActions::add_action`] to add
/// more actions. Create this struct with [`DataOfferDndActions::new`] or [`DataOfferDndAction::into`].
#[repr(transparent)]
pub struct DataOfferDndActions(u32);
impl DataOfferDndActions {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn add_action(mut self, action: DataOfferDndAction) -> Self {
        self.0 |= action as u32;
        self
    }

    pub fn add_in_place(&mut self, action: DataOfferDndAction) {
        self.0 |= action as u32;
    }
}
impl From<DataOfferDndAction> for DataOfferDndActions {
    fn from(value: DataOfferDndAction) -> Self {
        Self(value as u32)
    }
}
impl Default for DataOfferDndActions {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the source of a [`DataOffer`]. It's created and used by the client sending data.
pub struct DataSource {}
impl DataSource {
    /// Offer a MIME type to the other client. This can be called multiple times to offer multiple
    /// MIME types.
    fn offer(&self, mime_type: &str) {
        todo!()
    }

    /// Destroy this data source.
    fn destroy(self) {
        todo!()
    }

    /// Specify the available drag-and-drop actions when starting a drag-and-drop with another client.
    /// This can only be called once, before starting the drag-and-drop.
    fn set_actions(&self, actions: DataOfferDndActions) {
        todo!()
    }
}
pub trait DataSourceListener {
    /// Sent when the Wayland client receiving data accepts a MIME type.
    fn target(&mut self, mime_type: Option<&str>);

    /// A request to send the data to the other Wayland client.
    fn send(&mut self, mime_type: &str, fd: *mut ()); // note: we are responsible for closing the file descriptor

    /// Sent when the data transfer was cancelled. This data source and any associated data should then be cleaned up.
    fn cancelled(&mut self);

    /// Sent when the user has finished dropping a drag-and-drop. This doesn't mean that the other Wayland client
    /// accepted the drag-and-drop, just that the user finished dropping it.
    fn dnd_drop_performed(&mut self);

    /// The drag-and-drop finished. This data source and any associated data can now be deleted.
    fn dnd_finished(&mut self);

    /// The action that the compositor picked for the two clients to use to share data.
    fn dnd_action(&mut self, action: DataOfferDndAction);
}
#[repr(u32)]
pub enum DataSourceError {
    InvalidActionMask = 0,
    InvalidSource = 1,
}

/// The device used to start data transfers between Wayland clients. There is one [`DataDevice`] per seat, and it can
/// be obtained from the global [`DataDeviceManager`] singleton.
pub struct DataDevice {}
impl DataDevice {
    /// Request that the Wayland compositor start a drag-and-drop operation between this client and another surface this
    /// client has an implicit grab of (the implicit grab is specified in `serial`).
    ///
    /// `source` is the source of the data to transfer. If it's `None`, the client is responsible for transferring data on
    /// its own, and the other client will not receive events from this drag-and-drop. `origin` is the surface that the
    /// drag-and-drop started in. `icon` is an optional surface that will be dragged with the mouse cursor during the
    /// drag-and-drop.
    pub fn start_drag(
        &self,
        source: Option<DataSource>,
        origin: Surface,
        icon: Option<Surface>,
        serial: u32,
    ) {
        todo!()
    }

    /// Request that the Wayland compositor set a data source as the active selection. Passing `None` as the data source
    /// will clear the selection.
    pub fn set_selection(&self, source: Option<DataSource>, serial: u32) {
        todo!()
    }

    /// Destroy this [`DataDevice`].
    pub fn release(self) {
        todo!()
    }
}

pub struct DataDeviceManager {}
