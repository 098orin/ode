use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use ode_backend::Backend;
use ode_core::compositor::{Compositor, WindowId};
use smithay::{
    delegate_compositor, delegate_data_device, delegate_seat, delegate_shm, delegate_xdg_shell,
    input::{Seat, SeatHandler, SeatState},
    reexports::{
        calloop::EventLoop,
        wayland_server::{
            Client, Display, DisplayHandle, Resource,
            backend::{ClientData, ClientId, DisconnectReason},
            protocol::{wl_buffer, wl_seat, wl_surface::WlSurface},
        },
    },
    utils::Serial,
    wayland::{
        buffer::BufferHandler,
        compositor::{
            CompositorClientState, CompositorHandler, CompositorState, SurfaceAttributes,
            with_states,
        },
        selection::{
            SelectionHandler,
            data_device::{
                ClientDndGrabHandler, DataDeviceHandler, DataDeviceState, ServerDndGrabHandler,
            },
        },
        shell::xdg::{
            PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
        },
        shm::{ShmHandler, ShmState},
        socket::ListeningSocketSource,
    },
};

pub struct SmithayBackend {
    ode_compositor: Compositor,
}

impl SmithayBackend {
    pub fn new(ode_compositor: Compositor) -> Self {
        Self { ode_compositor }
    }
}

impl Backend for SmithayBackend {
    type Error = Box<dyn std::error::Error>;

    fn run(self) -> Result<(), Self::Error> {
        let mut event_loop = EventLoop::<CalloopData>::try_new()?;
        let display: Display<SmithayState> = Display::new()?;
        let dh = display.handle();

        let compositor_state = CompositorState::new::<SmithayState>(&dh);
        let shm_state = ShmState::new::<SmithayState>(&dh, vec![]);
        let mut seat_state = SeatState::new();
        let seat = seat_state.new_wl_seat(&dh, "ode");

        let state = SmithayState {
            smithay_compositor: compositor_state,
            xdg_shell_state: XdgShellState::new::<SmithayState>(&dh),
            shm_state,
            seat_state,
            data_device_state: DataDeviceState::new::<SmithayState>(&dh),
            seat,
            ode_compositor: self.ode_compositor,
            display_handle: dh,
            surface_map: HashMap::new(),
            client_windows: HashMap::new(),
        };

        let mut data = CalloopData { display, state };

        let listening_socket = ListeningSocketSource::new_auto()?;
        let socket_name = listening_socket.socket_name().to_os_string();
        println!("WAYLAND_DISPLAY={}", socket_name.to_string_lossy());

        event_loop
            .handle()
            .insert_source(listening_socket, |client_stream, _, data| {
                data.display
                    .handle()
                    .insert_client(client_stream, Arc::new(ClientState::default()))
                    .unwrap();
            })?;

        println!("Wayland compositor started.");

        event_loop.run(None, &mut data, |data| {
            data.display.dispatch_clients(&mut data.state).unwrap();
            data.display.flush_clients().unwrap();
        })?;

        Ok(())
    }
}

struct CalloopData {
    display: Display<SmithayState>,
    state: SmithayState,
}

struct SmithayState {
    smithay_compositor: CompositorState,
    xdg_shell_state: XdgShellState,
    shm_state: ShmState,
    seat_state: SeatState<Self>,
    data_device_state: DataDeviceState,
    seat: Seat<Self>,
    ode_compositor: Compositor,
    display_handle: DisplayHandle,
    surface_map: HashMap<WlSurface, WindowId>,
    client_windows: HashMap<ClientId, HashSet<WindowId>>,
}

impl BufferHandler for SmithayState {
    fn buffer_destroyed(&mut self, _buffer: &wl_buffer::WlBuffer) {
        todo!()
    }
}

impl SelectionHandler for SmithayState {
    type SelectionUserData = ();
}

impl XdgShellHandler for SmithayState {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        let window_id = self.ode_compositor.create_window();

        let client = self
            .display_handle
            .get_client(surface.wl_surface().id())
            .expect("toplevel surface has no client");

        self.client_windows
            .entry(client.id())
            .or_default()
            .insert(window_id);

        self.surface_map
            .insert(surface.wl_surface().clone(), window_id);

        surface.with_pending_state(|state| {
            state.size = Some((800, 600).into());
        });
        surface.send_configure();
    }
    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {
        todo!()
    }
    fn grab(&mut self, _surface: PopupSurface, _seat: wl_seat::WlSeat, _serial: Serial) {
        todo!()
    }
    fn reposition_request(
        &mut self,
        _surface: PopupSurface,
        _positioner: PositionerState,
        _token: u32,
    ) {
        todo!()
    }
}

impl CompositorHandler for SmithayState {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.smithay_compositor
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client.get_data::<ClientState>().unwrap().compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32;

        with_states(surface, |states| {
            for callback in states
                .cached_state
                .get::<SurfaceAttributes>()
                .current()
                .frame_callbacks
                .drain(..)
            {
                callback.done(time);
            }
        });
    }

    fn destroyed(&mut self, surface: &WlSurface) {
        if let Some(window_id) = self.surface_map.remove(surface) {
            self.ode_compositor.destroy_window(window_id);

            for windows in self.client_windows.values_mut() {
                windows.remove(&window_id);
            }
            self.client_windows.retain(|_, windows| !windows.is_empty());
        }
    }
}

impl ShmHandler for SmithayState {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

impl SeatHandler for SmithayState {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, _seat: &Seat<Self>, _focused: Option<&WlSurface>) {
        todo!()
    }
    fn cursor_image(
        &mut self,
        _seat: &Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {
        todo!()
    }
}

impl DataDeviceHandler for SmithayState {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.data_device_state
    }
}

impl ClientDndGrabHandler for SmithayState {}
impl ServerDndGrabHandler for SmithayState {}

#[derive(Default)]
struct ClientState {
    compositor_state: CompositorClientState,
}

impl ClientData for ClientState {
    fn initialized(&self, client_id: ClientId) {
        println!("client initialized: {:?}", client_id);
    }

    fn disconnected(&self, client_id: ClientId, reason: DisconnectReason) {
        println!("client disconnected: {client_id:?}, reason: {reason:?}");
    }
}

delegate_xdg_shell!(SmithayState);
delegate_compositor!(SmithayState);
delegate_shm!(SmithayState);
delegate_seat!(SmithayState);
delegate_data_device!(SmithayState);
